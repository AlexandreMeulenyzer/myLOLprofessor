use std::collections::HashSet;

use chrono::{TimeZone, Utc};
use serde::Serialize;
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

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MatchDetailParticipant {
    pub puuid: String,
    pub game_name: Option<String>,
    pub tag_line: Option<String>,
    pub champion_id: i64,
    pub champion_name: String,
    pub team_id: i64,
    pub team_position: String,
    pub win: bool,
    pub kills: i64,
    pub deaths: i64,
    pub assists: i64,
    pub champ_level: i64,
    pub gold_earned: i64,
    pub total_minions_killed: i64,
    pub neutral_minions_killed: i64,
    pub vision_score: i64,
    pub total_damage_dealt_to_champions: i64,
    pub total_damage_taken: i64,
    pub wards_placed: i64,
    pub wards_killed: i64,
    pub summoner1_id: i64,
    pub summoner2_id: i64,
    pub items: [i64; 7],
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MatchDetail {
    pub match_id: String,
    pub queue_id: i64,
    pub duration_seconds: i64,
    pub participants: Vec<MatchDetailParticipant>,
}

/// Detail complet d'un match (les 10 joueurs), recupere a la demande depuis
/// l'API Riot (un seul appel — pas de persistance locale, contrairement a
/// `sync_match_history` qui ne stocke que le participant synchronise).
/// Utilise par la vue "detail de match" (equivalent de la page de match
/// OP.GG/Porofessor).
#[tauri::command]
pub async fn get_match_detail(
    state: State<'_, AppState>,
    match_id: String,
    platform: String,
) -> Result<MatchDetail, String> {
    let riot_api = state
        .riot_api
        .read()
        .await
        .clone()
        .ok_or_else(|| "Aucune cle API Riot configuree.".to_string())?;

    let platform = Platform::from_str_loose(&platform)
        .ok_or_else(|| format!("Region inconnue: '{platform}'"))?;
    let route = platform.regional_route();

    let match_dto = riot_api
        .match_by_id(route, &match_id)
        .await
        .map_err(|err| err.to_string())?;

    let participants = match_dto
        .info
        .participants
        .into_iter()
        .map(|participant| MatchDetailParticipant {
            puuid: participant.puuid,
            game_name: participant.riot_id_game_name,
            tag_line: participant.riot_id_tagline,
            champion_id: participant.champion_id,
            champion_name: participant.champion_name,
            team_id: participant.team_id,
            team_position: participant.team_position,
            win: participant.win,
            kills: participant.kills,
            deaths: participant.deaths,
            assists: participant.assists,
            champ_level: participant.champ_level,
            gold_earned: participant.gold_earned,
            total_minions_killed: participant.total_minions_killed,
            neutral_minions_killed: participant.neutral_minions_killed,
            vision_score: participant.vision_score,
            total_damage_dealt_to_champions: participant.total_damage_dealt_to_champions,
            total_damage_taken: participant.total_damage_taken,
            wards_placed: participant.wards_placed,
            wards_killed: participant.wards_killed,
            summoner1_id: participant.summoner1_id,
            summoner2_id: participant.summoner2_id,
            items: [
                participant.item0,
                participant.item1,
                participant.item2,
                participant.item3,
                participant.item4,
                participant.item5,
                participant.item6,
            ],
        })
        .collect();

    Ok(MatchDetail {
        match_id: match_dto.metadata.match_id,
        queue_id: match_dto.info.queue_id,
        duration_seconds: match_dto.info.game_duration,
        participants,
    })
}
