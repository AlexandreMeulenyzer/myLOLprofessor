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
}

#[derive(Debug, Clone, Deserialize)]
pub struct ChampSelectSessionRaw {
    #[serde(rename = "localPlayerCellId")]
    pub local_player_cell_id: i64,
    #[serde(rename = "myTeam")]
    pub my_team: Vec<ChampSelectPlayer>,
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

/// `None` si le joueur n'est pas en champion select (404 attendu, pas une
/// erreur) ou si aucun champion n'a encore ete choisi.
pub async fn current_local_selection(
    client: &LcuClient,
) -> Result<Option<LocalPlayerSelection>, LcuError> {
    let session = match client
        .get_json::<ChampSelectSessionRaw>("/lol-champ-select/v1/session")
        .await
    {
        Ok(session) => session,
        Err(_) => return Ok(None),
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
}
