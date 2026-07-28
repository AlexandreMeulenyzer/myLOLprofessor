use tauri::State;

use crate::domain::coaching::{generate_report, CoachingReport, MatchPerformance};
use crate::infrastructure::db::matches_repository;
use crate::infrastructure::db::Database;
use crate::infrastructure::riot_api::dto::MatchParticipant;
use crate::stats_engine;

/// Genere un rapport de coaching pour un match deja synchronise localement
/// (voir `commands::history::sync_match_history`). `None` si le match n'est
/// pas connu pour ce joueur.
#[tauri::command]
pub fn get_coaching_report(
    db: State<'_, Database>,
    puuid: String,
    match_id: String,
) -> Result<Option<CoachingReport>, String> {
    let detail = {
        let conn = db.lock();
        matches_repository::get_participant(&conn, &match_id, &puuid)
            .map_err(|err| err.to_string())?
    };

    let Some(detail) = detail else {
        return Ok(None);
    };

    let participant = serde_json::from_str::<MatchParticipant>(&detail.stats_json)
        .map_err(|err| err.to_string())?;

    let minutes = (detail.duration_seconds as f64 / 60.0).max(1.0);
    let performance = MatchPerformance {
        kills: participant.kills,
        deaths: participant.deaths,
        assists: participant.assists,
        cs_per_min: (participant.total_minions_killed + participant.neutral_minions_killed) as f64
            / minutes,
        gold_per_min: participant.gold_earned as f64 / minutes,
        win: detail.win,
    };

    let baseline = {
        let conn = db.lock();
        stats_engine::compute_champion_role_stats(
            &conn,
            detail.champion_id,
            &detail.team_position,
            &detail.patch,
        )
        .map_err(|err| err.to_string())?
    };

    Ok(Some(generate_report(&performance, baseline.as_ref())))
}
