use serde::{Deserialize, Serialize};

use super::client::{LcuClient, LcuError};

#[derive(Debug, Clone, Deserialize)]
pub struct ChampSelectPlayer {
    #[serde(rename = "cellId")]
    pub cell_id: i64,
    #[serde(rename = "championId")]
    pub champion_id: i64,
    #[serde(rename = "assignedPosition", default)]
    pub assigned_position: String,
    /// Present sur les clients LCU recents (migration Riot vers les
    /// identifiants puuid). Absent/vide pour `theirTeam` selon la file de
    /// jeu (anonymisation partielle de certains modes).
    #[serde(default)]
    pub puuid: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ChampSelectSessionRaw {
    #[serde(rename = "localPlayerCellId")]
    pub local_player_cell_id: i64,
    #[serde(rename = "myTeam")]
    pub my_team: Vec<ChampSelectPlayer>,
    #[serde(rename = "theirTeam", default)]
    pub their_team: Vec<ChampSelectPlayer>,
}

/// Vue simplifiee de la session de champion select, du point de vue du
/// joueur local : le champion en cours de selection/verrouillage et le role
/// assigne (utilise pour interroger le stats engine avec la bonne cle
/// role/patch).
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LocalPlayerSelection {
    pub champion_id: i64,
    /// Normalise au format match-v5 (TOP/JUNGLE/MIDDLE/BOTTOM/UTILITY),
    /// vide si pas encore assigne par le client.
    pub role: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum TeamSide {
    Ally,
    Enemy,
}

/// Un participant de la partie en cours de composition, identifie par son
/// puuid — utilise par l'analyse d'equipe (Epic 4) pour recuperer rang,
/// maitrises et forme via l'API Riot.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ChampSelectParticipant {
    pub puuid: String,
    pub champion_id: i64,
    pub role: String,
    pub side: TeamSide,
    pub is_local_player: bool,
}

/// `None` si le joueur n'est pas en champion select (404 attendu, pas une
/// erreur) ou si aucun champion n'a encore ete choisi.
pub async fn current_local_selection(
    client: &LcuClient,
) -> Result<Option<LocalPlayerSelection>, LcuError> {
    let Some(session) = fetch_session(client).await? else {
        return Ok(None);
    };

    let local_player = session
        .my_team
        .iter()
        .find(|player| player.cell_id == session.local_player_cell_id);

    let Some(local_player) = local_player else {
        return Ok(None);
    };

    if local_player.champion_id == 0 {
        return Ok(None);
    }

    Ok(Some(LocalPlayerSelection {
        champion_id: local_player.champion_id,
        role: normalize_role(&local_player.assigned_position),
    }))
}

/// Tous les participants dont le puuid est visible (allies toujours,
/// adversaires selon la file de jeu). Les entrees sans champion choisi ou
/// sans puuid connu sont omises.
pub async fn current_participants(
    client: &LcuClient,
) -> Result<Vec<ChampSelectParticipant>, LcuError> {
    let Some(session) = fetch_session(client).await? else {
        return Ok(Vec::new());
    };

    let mut participants = Vec::new();

    for player in &session.my_team {
        if let Some(participant) =
            to_participant(player, TeamSide::Ally, session.local_player_cell_id)
        {
            participants.push(participant);
        }
    }

    for player in &session.their_team {
        if let Some(participant) =
            to_participant(player, TeamSide::Enemy, session.local_player_cell_id)
        {
            participants.push(participant);
        }
    }

    Ok(participants)
}

fn to_participant(
    player: &ChampSelectPlayer,
    side: TeamSide,
    local_player_cell_id: i64,
) -> Option<ChampSelectParticipant> {
    if player.puuid.is_empty() || player.champion_id == 0 {
        return None;
    }

    Some(ChampSelectParticipant {
        puuid: player.puuid.clone(),
        champion_id: player.champion_id,
        role: normalize_role(&player.assigned_position),
        side,
        is_local_player: player.cell_id == local_player_cell_id,
    })
}

async fn fetch_session(client: &LcuClient) -> Result<Option<ChampSelectSessionRaw>, LcuError> {
    match client
        .get_json::<ChampSelectSessionRaw>("/lol-champ-select/v1/session")
        .await
    {
        Ok(session) => Ok(Some(session)),
        Err(_) => Ok(None),
    }
}

fn normalize_role(raw: &str) -> String {
    match raw.to_ascii_lowercase().as_str() {
        "top" => "TOP".to_string(),
        "jungle" => "JUNGLE".to_string(),
        "middle" | "mid" => "MIDDLE".to_string(),
        "bottom" | "bot" | "adc" => "BOTTOM".to_string(),
        "utility" | "support" => "UTILITY".to_string(),
        _ => String::new(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalizes_known_roles_to_match_v5_convention() {
        assert_eq!(normalize_role("top"), "TOP");
        assert_eq!(normalize_role("middle"), "MIDDLE");
        assert_eq!(normalize_role("bottom"), "BOTTOM");
        assert_eq!(normalize_role("utility"), "UTILITY");
        assert_eq!(normalize_role(""), "");
        assert_eq!(normalize_role("unknown"), "");
    }

    #[test]
    fn skips_participants_without_puuid_or_champion() {
        let no_puuid = ChampSelectPlayer {
            cell_id: 1,
            champion_id: 103,
            assigned_position: "top".to_string(),
            puuid: String::new(),
        };
        assert!(to_participant(&no_puuid, TeamSide::Ally, 0).is_none());

        let no_champion = ChampSelectPlayer {
            cell_id: 1,
            champion_id: 0,
            assigned_position: "top".to_string(),
            puuid: "abc".to_string(),
        };
        assert!(to_participant(&no_champion, TeamSide::Ally, 0).is_none());
    }

    #[test]
    fn marks_local_player_by_matching_cell_id() {
        let player = ChampSelectPlayer {
            cell_id: 3,
            champion_id: 103,
            assigned_position: "middle".to_string(),
            puuid: "abc".to_string(),
        };
        let participant = to_participant(&player, TeamSide::Ally, 3).unwrap();
        assert!(participant.is_local_player);
        assert_eq!(participant.role, "MIDDLE");
    }
}
