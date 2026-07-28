use serde::Serialize;

use crate::infrastructure::live_client::{
    compute_objective_timers, LiveClientDataClient, ObjectiveTimers,
};

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LivePlayerSnapshot {
    pub summoner_name: String,
    pub champion_name: String,
    pub team: String,
    pub level: i64,
    pub kills: i64,
    pub deaths: i64,
    pub assists: i64,
    pub creep_score: i64,
    pub item_count: i64,
    pub is_dead: bool,
    pub respawn_timer_seconds: f64,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LiveGameSnapshot {
    pub game_time_seconds: f64,
    pub active_player_name: String,
    pub active_player_gold: f64,
    pub active_player_level: i64,
    pub objective_timers: ObjectiveTimers,
    pub contextual_tip: String,
    pub players: Vec<LivePlayerSnapshot>,
}

/// Interroge la Live Client Data API (`127.0.0.1:2999`), disponible
/// uniquement en partie. `None` si aucune partie n'est en cours — pas une
/// erreur, l'overlay doit simplement rester discret dans ce cas.
#[tauri::command]
pub async fn get_live_game_snapshot() -> Result<Option<LiveGameSnapshot>, String> {
    let client = LiveClientDataClient::new().map_err(|err| err.to_string())?;

    let data = match client.all_game_data().await {
        Ok(data) => data,
        Err(_) => return Ok(None),
    };

    let objective_timers = compute_objective_timers(&data.events.events, data.game_data.game_time);

    let players = data
        .all_players
        .into_iter()
        .map(|player| LivePlayerSnapshot {
            summoner_name: player.summoner_name,
            champion_name: player.champion_name,
            team: player.team,
            level: player.level,
            kills: player.scores.kills,
            deaths: player.scores.deaths,
            assists: player.scores.assists,
            creep_score: player.scores.creep_score,
            item_count: player.items.len() as i64,
            is_dead: player.is_dead,
            respawn_timer_seconds: player.respawn_timer,
        })
        .collect();

    Ok(Some(LiveGameSnapshot {
        active_player_name: data.active_player.summoner_name,
        active_player_gold: data.active_player.current_gold,
        active_player_level: data.active_player.level,
        contextual_tip: contextual_tip(&objective_timers, data.game_data.game_time),
        objective_timers,
        game_time_seconds: data.game_data.game_time,
        players,
    }))
}

fn contextual_tip(timers: &ObjectiveTimers, game_time: f64) -> String {
    const EARLY_GAME_END_SECONDS: f64 = 8.0 * 60.0;
    const SOON_THRESHOLD_SECONDS: f64 = 30.0;

    if timers.next_baron_seconds <= SOON_THRESHOLD_SECONDS && game_time > EARLY_GAME_END_SECONDS {
        "Le Baron sera bientôt disponible : gardez la vision autour de sa fosse.".to_string()
    } else if timers.next_dragon_seconds <= SOON_THRESHOLD_SECONDS {
        "Le prochain dragon sera bientôt disponible : préparez-vous à le contester.".to_string()
    } else if timers.herald_available {
        "Le Héraut est disponible : coordonnez-vous pour le prendre.".to_string()
    } else if game_time < EARLY_GAME_END_SECONDS {
        "Phase de lane : privilégiez le farm et évitez les échanges risqués.".to_string()
    } else if game_time >= 20.0 * 60.0 {
        "Le Baron est en jeu : gardez la vision et évitez les combats isolés.".to_string()
    } else {
        "Priorisez la vision et le farm avant le prochain objectif.".to_string()
    }
}
