use std::collections::HashSet;

use chrono::{TimeZone, Utc};
use tauri::State;

use crate::app_state::AppState;
use crate::infrastructure::db::matches_repository::{
    self, MatchHistoryEntry, MatchParticipantRecord,
};
use crate::infrastructure::db::Database;
use crate::infrastructure::riot_api::Platform;

/// Recupere les matchs recents d'un joueur depuis l'API Riot et les
/// persiste en base locale (une seule fois par match : les matchs deja
/// connus ne sont pas re-telecharges). Retourne le nombre de nouveaux
/// matchs enregistres.
#[tauri::command]
pub async fn sync_match_history(
    state: State<'_, AppState>,
    db: State<'_, Database>,
    puuid: String,
    platform: String,
    count: u32,
) -> Result<u32, String> {
    let riot_api = state
        .riot_api
        .read()
        .await
        .clone()
        .ok_or_else(|| "Aucune cle API Riot configuree.".to_string())?;

    let platform = Platform::from_str_loose(&platform)
        .ok_or_else(|| format!("Region inconnue: '{platform}'"))?;
    let route = platform.regional_route();

    let match_ids = riot_api
        .match_ids_by_puuid(route, &puuid, count)
        .await
        .map_err(|err| err.to_string())?;

    let known: HashSet<String> = matches_repository::known_match_ids(&db.lock(), &puuid)
        .map_err(|err| err.to_string())?
        .into_iter()
        .collect();

    let mut inserted = 0u32;

    for match_id in match_ids {
        if known.contains(&match_id) {
            continue;
        }

        let match_dto = riot_api
            .match_by_id(route, &match_id)
            .await
            .map_err(|err| err.to_string())?;

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

        let record = MatchParticipantRecord {
            match_id: match_dto.metadata.match_id,
            queue_id: match_dto.info.queue_id,
            patch,
            played_at,
            duration_seconds: match_dto.info.game_duration,
            banned_champion_ids,
            puuid: puuid.clone(),
            champion: participant.champion_name.clone(),
            champion_id: participant.champion_id,
            team_position: participant.team_position.clone(),
            win: participant.win,
            stats_json,
        };

        matches_repository::upsert_match_participant(&db.lock(), &record)
            .map_err(|err| err.to_string())?;
        inserted += 1;
    }

    Ok(inserted)
}

#[tauri::command]
pub fn get_match_history(
    db: State<'_, Database>,
    puuid: String,
    limit: u32,
) -> Result<Vec<MatchHistoryEntry>, String> {
    matches_repository::list_for_puuid(&db.lock(), &puuid, limit).map_err(|err| err.to_string())
}
