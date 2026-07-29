use std::sync::{Arc, Mutex};
use std::time::Duration;

use futures_util::StreamExt;
use tauri::{AppHandle, Emitter};

use crate::domain::GamePhase;

use super::client::LcuClient;
use super::process_discovery::{discover_lcu_credentials, LcuCredentials};
use super::websocket::{subscribe_events, RECONNECT_DELAY};
use super::{LcuConnection, LcuState};

pub const PHASE_CHANGED_EVENT: &str = "game-phase-changed";
const DISCOVERY_INTERVAL: Duration = Duration::from_secs(3);
const FAST_POLL_INTERVAL: Duration = Duration::from_millis(1500);
const SLOW_POLL_INTERVAL: Duration = Duration::from_secs(4);

pub type SharedPhase = Arc<Mutex<GamePhase>>;

/// Lance en tache de fond la detection automatique du client League of
/// Legends : decouverte du processus, connexion LCU, puis suivi de la
/// phase de jeu (WebSocket en priorite, polling REST en repli/redondance).
pub fn spawn(app: AppHandle, phase: SharedPhase, lcu_state: Arc<LcuState>) {
    tauri::async_runtime::spawn(async move {
        loop {
            let Some(credentials) = discover_lcu_credentials() else {
                *lcu_state.connection.write().await = None;
                apply_phase(&app, &phase, GamePhase::ClientClosed);
                tokio::time::sleep(DISCOVERY_INTERVAL).await;
                continue;
            };

            let Ok(client) = LcuClient::new(&credentials) else {
                tokio::time::sleep(DISCOVERY_INTERVAL).await;
                continue;
            };

            apply_phase(&app, &phase, GamePhase::ClientLaunched);

            match client.gameflow_phase().await {
                Ok(raw) => apply_phase(&app, &phase, GamePhase::from_lcu_str(&raw)),
                Err(_) => {
                    // Le processus existe mais l'API interne n'est pas encore
                    // prete (demarrage du client) : on retente bientot.
                    tokio::time::sleep(DISCOVERY_INTERVAL).await;
                    continue;
                }
            }

            *lcu_state.connection.write().await = Some(LcuConnection {
                credentials: credentials.clone(),
                client: client.clone(),
            });

            run_session(&app, &phase, &credentials, &client).await;

            *lcu_state.connection.write().await = None;
            apply_phase(&app, &phase, GamePhase::ClientClosed);
        }
    });
}

/// Suit la phase de jeu tant que le client reste joignable. Combine un flux
/// WebSocket (faible latence, best-effort) et un polling REST qui fait
/// office de source de verite et de filet de secours.
async fn run_session(
    app: &AppHandle,
    phase: &SharedPhase,
    credentials: &LcuCredentials,
    client: &LcuClient,
) {
    let ws_app = app.clone();
    let ws_phase = phase.clone();
    let ws_credentials = credentials.clone();

    let websocket_task = tauri::async_runtime::spawn(async move {
        loop {
            if discover_lcu_credentials().is_none() {
                break;
            }

            match subscribe_events(&ws_credentials).await {
                Ok(events) => {
                    futures_util::pin_mut!(events);
                    while let Some(Ok(event)) = events.next().await {
                        if event.uri == "/lol-gameflow/v1/gameflow-phase" {
                            if let Some(raw) = event.data.as_str() {
                                apply_phase(&ws_app, &ws_phase, GamePhase::from_lcu_str(raw));
                            }
                        }
                    }
                }
                Err(err) => {
                    log::debug!("connexion WebSocket LCU indisponible: {err}");
                }
            }

            tokio::time::sleep(RECONNECT_DELAY).await;
        }
    });

    loop {
        if discover_lcu_credentials().is_none() {
            break;
        }

        match client.gameflow_phase().await {
            Ok(raw) => apply_phase(app, phase, GamePhase::from_lcu_str(&raw)),
            Err(_) => break,
        }

        let current = *phase.lock().expect("phase mutex poisoned");
        tokio::time::sleep(poll_interval_for(current)).await;
    }

    websocket_task.abort();
}

fn poll_interval_for(phase: GamePhase) -> Duration {
    match phase {
        GamePhase::Lobby | GamePhase::ClientLaunched | GamePhase::ClientClosed => {
            SLOW_POLL_INTERVAL
        }
        _ => FAST_POLL_INTERVAL,
    }
}

fn apply_phase(app: &AppHandle, phase: &SharedPhase, new_phase: GamePhase) {
    let changed = {
        let mut guard = phase.lock().expect("phase mutex poisoned");
        if *guard == new_phase {
            false
        } else {
            *guard = new_phase;
            true
        }
    };

    if changed {
        log::info!("changement de phase de jeu: {new_phase:?}");
        let _ = app.emit(PHASE_CHANGED_EVENT, new_phase);
    }
}
