use std::sync::Arc;

use serde::Serialize;
use tauri::State;

use crate::app_state::AppState;
use crate::domain::mmr_estimate;
use crate::infrastructure::lcu::champ_select::{self, ChampSelectParticipant, TeamSide};
use crate::infrastructure::riot_api::{Platform, RiotApiClient};

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ChampionMasterySummary {
    pub champion_id: i64,
    pub champion_level: i64,
    pub champion_points: i64,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ParticipantAnalysis {
    pub puuid: String,
    pub game_name: String,
    pub tag_line: String,
    pub champion_id: i64,
    pub role: String,
    pub side: TeamSide,
    pub is_local_player: bool,
    pub tier: Option<String>,
    pub rank: Option<String>,
    pub league_points: Option<i64>,
    pub wins: Option<i64>,
    pub losses: Option<i64>,
    pub winrate_percent: Option<f64>,
    pub estimated_mmr: Option<i64>,
    pub top_champion_masteries: Vec<ChampionMasterySummary>,
    /// Signaux natifs Riot (league-v4), gratuits : serie de victoires en
    /// cours, "veteran" (100+ parties a ce rang cette saison) et "fresh
    /// blood" (nouvellement promu). Utile pour reperer un smurf potentiel
    /// (fresh_blood + WR eleve) sans appel API supplementaire.
    pub hot_streak: bool,
    pub veteran: bool,
    pub fresh_blood: bool,
    /// Heuristique "One Trick Pony" : part des points de maitrise concentres
    /// sur le champion le plus joue, parmi les maitrises recuperees. Calcule
    /// a partir de donnees deja recuperees (aucun appel API supplementaire),
    /// contrairement a une detection basee sur l'historique de matchs.
    pub top_champion_mastery_share_percent: Option<f64>,
    /// Vrai si le champion choisi correspond au champion de plus haute
    /// maitrise du joueur.
    pub is_playing_top_mastery_champion: bool,
}

const TOP_MASTERIES_LIMIT: usize = 3;

/// Analyse les 10 joueurs de la partie en cours de composition (champion
/// select) : rang, maitrises principales, MMR estime. Les participants
/// pour lesquels l'identite n'est pas encore connue du client (`puuid`
/// absent) sont omis silencieusement. Tolerant aux erreurs individuelles :
/// un participant dont la recherche echoue est simplement absent du
/// resultat plutot que de faire echouer toute l'analyse.
#[tauri::command]
pub async fn get_champ_select_team_analysis(
    state: State<'_, AppState>,
    platform: String,
) -> Result<Vec<ParticipantAnalysis>, String> {
    let riot_api = state
        .riot_api
        .read()
        .await
        .clone()
        .ok_or_else(|| "Aucune cle API Riot configuree.".to_string())?;

    let platform = Platform::from_str_loose(&platform)
        .ok_or_else(|| format!("Region inconnue: '{platform}'"))?;

    let participants = {
        let connection_guard = state.lcu.connection.read().await;
        let Some(connection) = connection_guard.as_ref() else {
            return Ok(Vec::new());
        };
        champ_select::current_participants(&connection.client)
            .await
            .map_err(|err| err.to_string())?
    };

    let analyses = futures_util::future::join_all(
        participants
            .into_iter()
            .map(|participant| analyze_participant(riot_api.clone(), platform, participant)),
    )
    .await;

    Ok(analyses.into_iter().flatten().collect())
}

async fn analyze_participant(
    riot_api: Arc<RiotApiClient>,
    platform: Platform,
    participant: ChampSelectParticipant,
) -> Option<ParticipantAnalysis> {
    let account = riot_api
        .account_by_puuid(platform.regional_route(), &participant.puuid)
        .await
        .ok()?;

    let league_entries = riot_api
        .league_entries_by_puuid(platform, &participant.puuid)
        .await
        .unwrap_or_default();
    let masteries = riot_api
        .champion_masteries_by_puuid(platform, &participant.puuid)
        .await
        .unwrap_or_default();

    let solo_queue = league_entries
        .iter()
        .find(|entry| entry.queue_type == "RANKED_SOLO_5x5");

    let mut top_champion_masteries: Vec<ChampionMasterySummary> = masteries
        .into_iter()
        .map(|mastery| ChampionMasterySummary {
            champion_id: mastery.champion_id,
            champion_level: mastery.champion_level,
            champion_points: mastery.champion_points,
        })
        .collect();
    top_champion_masteries.sort_by(|a, b| b.champion_points.cmp(&a.champion_points));

    let total_mastery_points: i64 = top_champion_masteries
        .iter()
        .map(|m| m.champion_points)
        .sum();
    let top_champion_mastery_share_percent = top_champion_masteries.first().and_then(|top| {
        if total_mastery_points == 0 {
            None
        } else {
            Some(
                ((top.champion_points as f64 / total_mastery_points as f64) * 1000.0).round()
                    / 10.0,
            )
        }
    });
    let is_playing_top_mastery_champion = top_champion_masteries
        .first()
        .is_some_and(|top| top.champion_id == participant.champion_id);

    top_champion_masteries.truncate(TOP_MASTERIES_LIMIT);

    Some(ParticipantAnalysis {
        puuid: participant.puuid,
        game_name: account.game_name.unwrap_or_default(),
        tag_line: account.tag_line.unwrap_or_default(),
        champion_id: participant.champion_id,
        role: participant.role,
        side: participant.side,
        is_local_player: participant.is_local_player,
        tier: solo_queue.map(|entry| entry.tier.clone()),
        rank: solo_queue.map(|entry| entry.rank.clone()),
        league_points: solo_queue.map(|entry| entry.league_points),
        wins: solo_queue.map(|entry| entry.wins),
        losses: solo_queue.map(|entry| entry.losses),
        winrate_percent: solo_queue.map(|entry| {
            let games = entry.wins + entry.losses;
            if games == 0 {
                0.0
            } else {
                ((entry.wins as f64 / games as f64) * 1000.0).round() / 10.0
            }
        }),
        estimated_mmr: solo_queue.and_then(|entry| {
            mmr_estimate::estimate_mmr(&entry.tier, &entry.rank, entry.league_points)
        }),
        hot_streak: solo_queue.is_some_and(|entry| entry.hot_streak),
        veteran: solo_queue.is_some_and(|entry| entry.veteran),
        fresh_blood: solo_queue.is_some_and(|entry| entry.fresh_blood),
        top_champion_mastery_share_percent,
        is_playing_top_mastery_champion,
        top_champion_masteries,
    })
}
