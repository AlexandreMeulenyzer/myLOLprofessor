mod app_state;
mod commands;
mod domain;
mod infrastructure;
mod stats_engine;

use std::sync::Arc;

use tauri::{Listener, Manager};

use app_state::AppState;
use domain::GamePhase;
use infrastructure::data_dragon::DataDragonClient;
use infrastructure::db::Database;
use infrastructure::lcu::watcher;
use infrastructure::riot_api::RiotApiClient;
use infrastructure::secure_storage;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_single_instance::init(|app, _args, _cwd| {
            if let Some(window) = app.get_webview_window("main") {
                let _ = window.set_focus();
            }
        }))
        .plugin(
            tauri_plugin_log::Builder::default()
                .level(if cfg!(debug_assertions) {
                    log::LevelFilter::Debug
                } else {
                    log::LevelFilter::Info
                })
                .build(),
        )
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_store::Builder::default().build())
        .plugin(tauri_plugin_opener::init())
        .manage(AppState::new())
        .invoke_handler(tauri::generate_handler![
            commands::accounts::link_account,
            commands::accounts::list_accounts,
            commands::accounts::remove_account,
            commands::accounts::set_primary_account,
            commands::champion_select::get_current_champ_select_selection,
            commands::coaching::get_coaching_report,
            commands::game_state::get_game_phase,
            commands::history::get_match_history,
            commands::history::sync_match_history,
            commands::live_game::get_live_game_snapshot,
            commands::objectives::create_objective,
            commands::objectives::delete_objective,
            commands::objectives::list_objectives_with_progress,
            commands::overlay::toggle_overlay_window,
            commands::profile::get_profile,
            commands::profile::get_lp_history,
            commands::riot_api_key::save_riot_api_key,
            commands::riot_api_key::has_riot_api_key,
            commands::riot_api_key::delete_riot_api_key,
            commands::search::resolve_summoner,
            commands::static_data::get_latest_patch_version,
            commands::static_data::get_champions,
            commands::static_data::get_champion_detail,
            commands::static_data::get_items,
            commands::static_data::get_runes,
            commands::static_data::get_summoner_spells,
            commands::stats::get_champion_role_stats,
            commands::team_analysis::get_champ_select_team_analysis
        ])
        .setup(|app| {
            let app_data_dir = app.path().app_data_dir()?;
            std::fs::create_dir_all(&app_data_dir)?;
            let database = Database::open(&app_data_dir.join("wardstone.sqlite"))?;
            app.manage(database);
            app.manage(DataDragonClient::new(app_data_dir.join("ddragon")));

            let state = app.state::<AppState>();
            watcher::spawn(app.handle().clone(), state.phase.clone(), state.lcu.clone());

            // Ouvre/ferme automatiquement l'overlay en fonction de la phase
            // de jeu : visible uniquement pendant une partie en cours. Un
            // utilisateur peut toujours la fermer manuellement en cours de
            // partie (elle ne se rouvrira qu'a la prochaine transition).
            let overlay_app_handle = app.handle().clone();
            app.listen(watcher::PHASE_CHANGED_EVENT, move |event| {
                let Ok(phase) = serde_json::from_str::<GamePhase>(event.payload()) else {
                    return;
                };

                let result = if phase == GamePhase::InProgress {
                    commands::overlay::show_overlay_window(&overlay_app_handle)
                } else {
                    commands::overlay::hide_overlay_window(&overlay_app_handle)
                };

                if let Err(err) = result {
                    log::warn!("gestion automatique de l'overlay echouee: {err}");
                }
            });

            // Reactive une cle API Riot deja enregistree lors d'une session
            // precedente (le trousseau OS est la source de verite ; l'etat
            // en memoire n'est qu'un cache pour la session en cours).
            let app_handle = app.handle().clone();
            tauri::async_runtime::spawn(async move {
                if let Ok(Some(api_key)) = secure_storage::get_riot_api_key() {
                    if let Ok(client) = RiotApiClient::new(api_key) {
                        let state = app_handle.state::<AppState>();
                        *state.riot_api.write().await = Some(Arc::new(client));
                    }
                }
            });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("erreur au lancement de l'application Tauri");
}
