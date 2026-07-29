use serde::{Deserialize, Serialize};

/// account-v1 : `/riot/account/v1/accounts/by-riot-id/{gameName}/{tagLine}`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountDto {
    pub puuid: String,
    #[serde(default)]
    pub game_name: Option<String>,
    #[serde(default)]
    pub tag_line: Option<String>,
}

/// summoner-v4 : `/lol/summoner/v4/summoners/by-puuid/{puuid}`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SummonerDto {
    pub puuid: String,
    pub profile_icon_id: i64,
    pub revision_date: i64,
    pub summoner_level: i64,
}

/// league-v4 : `/lol/league/v4/entries/by-puuid/{puuid}`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LeagueEntryDto {
    #[serde(default)]
    pub league_id: Option<String>,
    pub queue_type: String,
    pub tier: String,
    pub rank: String,
    pub league_points: i64,
    pub wins: i64,
    pub losses: i64,
    #[serde(default)]
    pub hot_streak: bool,
    #[serde(default)]
    pub veteran: bool,
    #[serde(default)]
    pub fresh_blood: bool,
    #[serde(default)]
    pub inactive: bool,
}

/// champion-mastery-v4 : `/lol/champion-mastery/v4/champion-masteries/by-puuid/{puuid}`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChampionMasteryDto {
    pub champion_id: i64,
    pub champion_level: i64,
    pub champion_points: i64,
    #[serde(default)]
    pub last_play_time: i64,
    #[serde(default)]
    pub tokens_earned: i64,
}

/// match-v5 : `/lol/match/v5/matches/{matchId}`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MatchDto {
    pub metadata: MatchMetadata,
    pub info: MatchInfo,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MatchMetadata {
    pub match_id: String,
    pub participants: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MatchInfo {
    pub game_creation: i64,
    pub game_duration: i64,
    pub game_version: String,
    pub queue_id: i64,
    pub participants: Vec<MatchParticipant>,
    pub teams: Vec<MatchTeam>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MatchParticipant {
    pub puuid: String,
    #[serde(default)]
    pub riot_id_game_name: Option<String>,
    #[serde(default)]
    pub riot_id_tagline: Option<String>,
    pub champion_name: String,
    pub champion_id: i64,
    #[serde(default)]
    pub team_position: String,
    pub team_id: i64,
    pub win: bool,
    pub kills: i64,
    pub deaths: i64,
    pub assists: i64,
    pub champ_level: i64,
    pub gold_earned: i64,
    pub total_minions_killed: i64,
    #[serde(default)]
    pub neutral_minions_killed: i64,
    #[serde(default)]
    pub vision_score: i64,
    #[serde(default)]
    pub total_damage_dealt_to_champions: i64,
    #[serde(default)]
    pub total_damage_taken: i64,
    #[serde(default)]
    pub wards_placed: i64,
    #[serde(default)]
    pub wards_killed: i64,
    pub summoner1_id: i64,
    pub summoner2_id: i64,
    pub item0: i64,
    pub item1: i64,
    pub item2: i64,
    pub item3: i64,
    pub item4: i64,
    pub item5: i64,
    pub item6: i64,
    #[serde(default)]
    pub perks: Option<Perks>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Perks {
    pub stat_perks: PerkStatShards,
    pub styles: Vec<PerkStyle>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PerkStatShards {
    pub offense: i64,
    pub flex: i64,
    pub defense: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PerkStyle {
    pub description: String,
    pub style: i64,
    pub selections: Vec<PerkSelection>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PerkSelection {
    pub perk: i64,
    #[serde(default)]
    pub var1: i64,
    #[serde(default)]
    pub var2: i64,
    #[serde(default)]
    pub var3: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MatchTeam {
    pub team_id: i64,
    pub win: bool,
    #[serde(default)]
    pub bans: Vec<MatchBan>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MatchBan {
    pub champion_id: i64,
    pub pick_turn: i64,
}

/// spectator-v5 : `/lol/spectator/v5/active-games/by-summoner/{puuid}`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CurrentGameInfo {
    pub game_id: i64,
    pub game_mode: String,
    #[serde(default)]
    pub game_length: i64,
    pub participants: Vec<CurrentGameParticipant>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CurrentGameParticipant {
    pub puuid: String,
    pub champion_id: i64,
    pub team_id: i64,
    #[serde(default)]
    pub summoner_name: String,
}
