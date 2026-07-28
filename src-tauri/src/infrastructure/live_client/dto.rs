use serde::Deserialize;

/// `GET https://127.0.0.1:2999/liveclientdata/allgamedata` — API officielle
/// Riot exposee localement pendant une partie en cours. Reference:
/// https://developer.riotgames.com/docs/lol#game-client-api
#[derive(Debug, Clone, Deserialize)]
pub struct AllGameData {
    #[serde(rename = "activePlayer")]
    pub active_player: ActivePlayer,
    #[serde(rename = "allPlayers")]
    pub all_players: Vec<PlayerData>,
    pub events: EventsWrapper,
    #[serde(rename = "gameData")]
    pub game_data: GameData,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ActivePlayer {
    #[serde(rename = "summonerName", default)]
    pub summoner_name: String,
    #[serde(rename = "currentGold", default)]
    pub current_gold: f64,
    pub level: i64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PlayerData {
    #[serde(rename = "summonerName", default)]
    pub summoner_name: String,
    #[serde(rename = "championName")]
    pub champion_name: String,
    pub team: String,
    pub level: i64,
    pub scores: PlayerScores,
    /// Seul le nombre d'objets nous interesse actuellement ; le contenu
    /// brut est conserve tel quel plutot que type integralement.
    #[serde(default)]
    pub items: Vec<serde_json::Value>,
    #[serde(rename = "isDead", default)]
    pub is_dead: bool,
    #[serde(rename = "respawnTimer", default)]
    pub respawn_timer: f64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PlayerScores {
    pub kills: i64,
    pub deaths: i64,
    pub assists: i64,
    #[serde(rename = "creepScore", default)]
    pub creep_score: i64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct EventsWrapper {
    #[serde(rename = "Events", default)]
    pub events: Vec<GameEvent>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct GameEvent {
    #[serde(rename = "EventName")]
    pub event_name: String,
    #[serde(rename = "EventTime")]
    pub event_time: f64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct GameData {
    #[serde(rename = "gameTime")]
    pub game_time: f64,
}
