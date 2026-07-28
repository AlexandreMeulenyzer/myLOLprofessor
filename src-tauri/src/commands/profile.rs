use serde::Serialize;
use tauri::State;

use crate::app_state::AppState;
use crate::domain::mmr_estimate;
use crate::infrastructure::riot_api::Platform;

#[derive(Debug, Serialize)]
pub struct ProfileSummary {
    pub puuid: String,
    pub profile_icon_id: i64,
    pub summoner_level: i64,
    pub league_entries: Vec<LeagueEntrySummary>,
    pub top_champions: Vec<ChampionMasterySummary>,
}

#[derive(Debug, Serialize)]
pub struct LeagueEntrySummary {
    pub queue_type: String,
    pub tier: String,
    pub rank: String,
    pub league_points: i64,
    pub wins: i64,
    pub losses: i64,
    pub winrate_percent: f64,
    /// Estimation heuristique (Riot ne publie pas de MMR) — voir
    /// `domain::mmr_estimate`. `None` pour les files non classees.
    pub estimated_mmr: Option<i64>,
}

#[derive(Debug, Serialize)]
pub struct ChampionMasterySummary {
    pub champion_id: i64,
    pub champion_level: i64,
    pub champion_points: i64,
}

const TOP_CHAMPIONS_LIMIT: usize = 5;

#[tauri::command]
pub async fn get_profile(
    state: State<'_, AppState>,
    puuid: String,
    platform: String,
) -> Result<ProfileSummary, String> {
    let riot_api = state
        .riot_api
        .read()
        .await
        .clone()
        .ok_or_else(|| "Aucune cle API Riot configuree.".to_string())?;

    let platform = Platform::from_str_loose(&platform)
        .ok_or_else(|| format!("Region inconnue: '{platform}'"))?;

    let summoner = riot_api
        .summoner_by_puuid(platform, &puuid)
        .await
        .map_err(|err| err.to_string())?;
    let league_entries = riot_api
        .league_entries_by_puuid(platform, &puuid)
        .await
        .map_err(|err| err.to_string())?;
    let masteries = riot_api
        .champion_masteries_by_puuid(platform, &puuid)
        .await
        .map_err(|err| err.to_string())?;

    let mut top_champions: Vec<ChampionMasterySummary> = masteries
        .into_iter()
        .map(|mastery| ChampionMasterySummary {
            champion_id: mastery.champion_id,
            champion_level: mastery.champion_level,
            champion_points: mastery.champion_points,
        })
        .collect();
    top_champions.sort_by(|a, b| b.champion_points.cmp(&a.champion_points));
    top_champions.truncate(TOP_CHAMPIONS_LIMIT);

    let league_entries = league_entries
        .into_iter()
        .map(|entry| {
            let games_played = entry.wins + entry.losses;
            let winrate_percent = if games_played > 0 {
                (entry.wins as f64 / games_played as f64) * 100.0
            } else {
                0.0
            };

            LeagueEntrySummary {
                estimated_mmr: mmr_estimate::estimate_mmr(
                    &entry.tier,
                    &entry.rank,
                    entry.league_points,
                ),
                queue_type: entry.queue_type,
                tier: entry.tier,
                rank: entry.rank,
                league_points: entry.league_points,
                wins: entry.wins,
                losses: entry.losses,
                winrate_percent: (winrate_percent * 10.0).round() / 10.0,
            }
        })
        .collect();

    Ok(ProfileSummary {
        puuid,
        profile_icon_id: summoner.profile_icon_id,
        summoner_level: summoner.summoner_level,
        league_entries,
        top_champions,
    })
}
