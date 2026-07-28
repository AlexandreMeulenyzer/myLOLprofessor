use chrono::Utc;
use serde::Serialize;
use tauri::State;

use crate::app_state::AppState;
use crate::domain::objective::{
    evaluate_progress, ObjectiveContext, ObjectiveKind, ObjectiveProgress,
};
use crate::infrastructure::db::matches_repository;
use crate::infrastructure::db::objectives_repository;
use crate::infrastructure::db::Database;
use crate::infrastructure::riot_api::Platform;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ObjectiveWithProgress {
    pub id: i64,
    pub kind: ObjectiveKind,
    pub created_at: String,
    pub achieved_at: Option<String>,
    pub progress: ObjectiveProgress,
}

#[tauri::command]
pub fn create_objective(
    db: State<'_, Database>,
    puuid: String,
    kind: ObjectiveKind,
) -> Result<i64, String> {
    let kind_json = serde_json::to_string(&kind).map_err(|err| err.to_string())?;
    let created_at = Utc::now().to_rfc3339();
    objectives_repository::insert(&db.lock(), &puuid, kind.tag(), &kind_json, &created_at)
        .map_err(|err| err.to_string())
}

#[tauri::command]
pub fn delete_objective(db: State<'_, Database>, id: i64, puuid: String) -> Result<(), String> {
    objectives_repository::delete(&db.lock(), id, &puuid).map_err(|err| err.to_string())
}

/// Liste les objectifs de l'utilisateur avec leur progression actuelle.
/// Recupere le rang courant une seule fois (partage entre tous les
/// objectifs) pour limiter les appels a l'API Riot. Marque automatiquement
/// un objectif comme atteint la premiere fois que sa condition est remplie.
#[tauri::command]
pub async fn list_objectives_with_progress(
    state: State<'_, AppState>,
    db: State<'_, Database>,
    puuid: String,
    platform: String,
) -> Result<Vec<ObjectiveWithProgress>, String> {
    let records =
        objectives_repository::list_for_puuid(&db.lock(), &puuid).map_err(|err| err.to_string())?;

    let league_entries = if let Some(riot_api) = state.riot_api.read().await.clone() {
        if let Some(platform) = Platform::from_str_loose(&platform) {
            riot_api
                .league_entries_by_puuid(platform, &puuid)
                .await
                .unwrap_or_default()
        } else {
            Vec::new()
        }
    } else {
        Vec::new()
    };

    let now = Utc::now().to_rfc3339();
    let mut results = Vec::with_capacity(records.len());

    for record in records {
        let Ok(kind) = serde_json::from_str::<ObjectiveKind>(&record.kind_json) else {
            continue;
        };

        let queue_type = match &kind {
            ObjectiveKind::ReachRank { queue_type, .. }
            | ObjectiveKind::WinrateTarget { queue_type, .. } => Some(queue_type.as_str()),
            ObjectiveKind::GamesPlayed { .. } => None,
        };

        let entry = queue_type.and_then(|queue| {
            league_entries
                .iter()
                .find(|entry| entry.queue_type == queue)
        });

        let games_played_since_creation =
            matches_repository::count_since(&db.lock(), &puuid, &record.created_at).unwrap_or(0);

        let context = ObjectiveContext {
            current_tier: entry.map(|e| e.tier.clone()),
            current_rank: entry.map(|e| e.rank.clone()),
            current_league_points: entry.map(|e| e.league_points).unwrap_or(0),
            current_wins: entry.map(|e| e.wins).unwrap_or(0),
            current_losses: entry.map(|e| e.losses).unwrap_or(0),
            games_played_since_creation,
        };

        let progress = evaluate_progress(&kind, &context);

        if progress.achieved && record.achieved_at.is_none() {
            let _ = objectives_repository::mark_achieved(&db.lock(), record.id, &now);
        }

        results.push(ObjectiveWithProgress {
            id: record.id,
            kind,
            created_at: record.created_at,
            achieved_at: if progress.achieved && record.achieved_at.is_none() {
                Some(now.clone())
            } else {
                record.achieved_at
            },
            progress,
        });
    }

    Ok(results)
}
