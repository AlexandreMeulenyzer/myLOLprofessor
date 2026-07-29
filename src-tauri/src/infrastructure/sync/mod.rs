use std::collections::HashSet;
use std::time::Duration;

use chrono::{TimeZone, Utc};
use tauri::{AppHandle, Emitter, Manager};

use crate::app_state::AppState;
use crate::infrastructure::db::accounts_repository;
use crate::infrastructure::db::league_snapshots_repository::{self, LeagueSnapshotInput};
use crate::infrastructure::db::matches_repository::{self, MatchParticipantRecord};
use crate::infrastructure::db::Database;
use crate::infrastructure::riot_api::{Platform, RiotApiClient};

/// Emis a chaque compte lie dont le rang ou l'historique vient d'etre
/// rafraichi en tache de fond, avec le puuid concerne en payload. Le
/// frontend l'ecoute pour invalider ses caches React Query sans action de
/// l'utilisateur (voir `useAccountSyncListener`).
pub const ACCOUNT_SYNCED_EVENT: &str = "account-synced";

const PERIODIC_SYNC_INTERVAL: Duration = Duration::from_secs(15 * 60);
/// L'API match-v5 ne referme la partie que quelques dizaines de secondes
/// apres la fin de celle-ci ; un delai plus court renverrait systematiquement
/// une liste sans le dernier match.
const POST_GAME_SYNC_DELAY: Duration = Duration::from_secs(45);
const MATCH_SYNC_COUNT: u32 = 10;

#[derive(Debug, Default)]
pub struct SyncOutcome {
    pub matches_synced: u32,
    pub rank_updated: bool,
}

/// Rafraichit le rang (avec historique LP deduplique) et l'historique de
/// matchs d'un compte lie. Coeur partage par la synchronisation periodique
/// et par le declenchement en fin de partie.
pub async fn sync_account(
    riot_api: &RiotApiClient,
    db: &Database,
    puuid: &str,
    platform: Platform,
) -> Result<SyncOutcome, String> {
    let mut outcome = SyncOutcome::default();

    let league_entries = riot_api
        .league_entries_by_puuid(platform, puuid)
        .await
        .map_err(|err| err.to_string())?;

    let captured_at = Utc::now().to_rfc3339();
    {
        let conn = db.lock();
        for entry in &league_entries {
            let changed =
                league_snapshots_repository::latest_for_puuid(&conn, puuid, &entry.queue_type)
                    .map_err(|err| err.to_string())?
                    .map(|latest| {
                        latest.tier != entry.tier
                            || latest.rank != entry.rank
                            || latest.league_points != entry.league_points
                    })
                    .unwrap_or(true);

            if changed {
                league_snapshots_repository::record_snapshot(
                    &conn,
                    LeagueSnapshotInput {
                        puuid,
                        queue_type: &entry.queue_type,
                        tier: &entry.tier,
                        rank: &entry.rank,
                        league_points: entry.league_points,
                        wins: entry.wins,
                        losses: entry.losses,
                        captured_at: &captured_at,
                    },
                )
                .map_err(|err| err.to_string())?;
                outcome.rank_updated = true;
            }
        }
    }

    let route = platform.regional_route();
    let match_ids = riot_api
        .match_ids_by_puuid(route, puuid, MATCH_SYNC_COUNT)
        .await
        .map_err(|err| err.to_string())?;

    let known: HashSet<String> = matches_repository::known_match_ids(&db.lock(), puuid)
        .map_err(|err| err.to_string())?
        .into_iter()
        .collect();

    for match_id in match_ids {
        if known.contains(&match_id) {
            continue;
        }

        // Best-effort : un match individuel indisponible ne doit pas faire
        // echouer tout le cycle de synchronisation, on le retentera au
        // prochain passage.
        let Ok(match_dto) = riot_api.match_by_id(route, &match_id).await else {
            continue;
        };

        let Some(participant) = match_dto
            .info
            .participants
            .iter()
            .find(|participant| participant.puuid == puuid)
        else {
            continue;
        };

        let played_at = Utc
            .timestamp_millis_opt(match_dto.info.game_creation)
            .single()
            .unwrap_or_else(Utc::now)
            .to_rfc3339();

        let patch = match_dto
            .info
            .game_version
            .split('.')
            .take(2)
            .collect::<Vec<_>>()
            .join(".");

        let stats_json = serde_json::to_string(participant).map_err(|err| err.to_string())?;
        let banned_champion_ids = match_dto
            .info
            .teams
            .iter()
            .flat_map(|team| team.bans.iter().map(|ban| ban.champion_id))
            .collect();

        matches_repository::upsert_match_participant(
            &db.lock(),
            &MatchParticipantRecord {
                match_id: match_dto.metadata.match_id,
                queue_id: match_dto.info.queue_id,
                patch,
                played_at,
                duration_seconds: match_dto.info.game_duration,
                banned_champion_ids,
                puuid: puuid.to_string(),
                champion: participant.champion_name.clone(),
                champion_id: participant.champion_id,
                team_position: participant.team_position.clone(),
                win: participant.win,
                stats_json,
            },
        )
        .map_err(|err| err.to_string())?;

        outcome.matches_synced += 1;
    }

    Ok(outcome)
}

async fn sync_all_linked_accounts(app: &AppHandle) {
    let state = app.state::<AppState>();
    let Some(riot_api) = state.riot_api.read().await.clone() else {
        return;
    };
    let db = app.state::<Database>();

    let accounts = match accounts_repository::list(&db.lock()) {
        Ok(accounts) => accounts,
        Err(err) => {
            log::warn!(
                "liste des comptes lies indisponible pour la synchronisation automatique: {err}"
            );
            return;
        }
    };

    for account in accounts {
        let Some(platform) = Platform::from_str_loose(&account.platform) else {
            continue;
        };

        match sync_account(&riot_api, &db, &account.puuid, platform).await {
            Ok(outcome) if outcome.matches_synced > 0 || outcome.rank_updated => {
                log::info!(
                    "synchronisation automatique de {}#{}: {} match(s), rang {}",
                    account.game_name,
                    account.tag_line,
                    outcome.matches_synced,
                    if outcome.rank_updated {
                        "mis a jour"
                    } else {
                        "inchange"
                    }
                );
                let _ = app.emit(ACCOUNT_SYNCED_EVENT, &account.puuid);
            }
            Ok(_) => {}
            Err(err) => {
                log::debug!(
                    "synchronisation automatique echouee pour {}#{}: {err}",
                    account.game_name,
                    account.tag_line
                );
            }
        }
    }
}

/// Lance en tache de fond la synchronisation periodique de tous les comptes
/// lies (rang + historique de matchs), independamment de toute navigation de
/// l'utilisateur dans l'application. Un premier cycle s'execute des le
/// demarrage pour eviter d'attendre le premier intervalle complet.
pub fn spawn_periodic(app: AppHandle) {
    tauri::async_runtime::spawn(async move {
        loop {
            sync_all_linked_accounts(&app).await;
            tokio::time::sleep(PERIODIC_SYNC_INTERVAL).await;
        }
    });
}

/// Synchronisation ponctuelle declenchee a la transition de phase
/// `EndOfGame` : rafraichit l'historique et le rang de tous les comptes lies
/// apres un court delai (voir `POST_GAME_SYNC_DELAY`).
pub fn spawn_post_game(app: AppHandle) {
    tauri::async_runtime::spawn(async move {
        tokio::time::sleep(POST_GAME_SYNC_DELAY).await;
        sync_all_linked_accounts(&app).await;
    });
}
