use serde::{Deserialize, Serialize};

/// Phase de la partie telle que rapportee par le League Client Update (LCU).
/// Miroir de `src/shared/types/game-phase.ts` cote frontend.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum GamePhase {
    ClientClosed,
    ClientLaunched,
    Lobby,
    Matchmaking,
    ReadyCheck,
    ChampSelect,
    GameStart,
    InProgress,
    WaitingForStats,
    EndOfGame,
}

impl GamePhase {
    /// Traduit la valeur brute renvoyee par `/lol-gameflow/v1/gameflow-phase`.
    /// Reference : https://developer.riotgames.com/docs/lol#league-client-api
    pub fn from_lcu_str(raw: &str) -> Self {
        match raw {
            "None" => GamePhase::Lobby,
            "Lobby" => GamePhase::Lobby,
            "Matchmaking" => GamePhase::Matchmaking,
            "ReadyCheck" => GamePhase::ReadyCheck,
            "ChampSelect" => GamePhase::ChampSelect,
            "GameStart" => GamePhase::GameStart,
            "InProgress" => GamePhase::InProgress,
            "WaitingForStats" => GamePhase::WaitingForStats,
            "PreEndOfGame" | "EndOfGame" => GamePhase::EndOfGame,
            _ => GamePhase::Lobby,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn maps_known_lcu_phases() {
        assert_eq!(
            GamePhase::from_lcu_str("ChampSelect"),
            GamePhase::ChampSelect
        );
        assert_eq!(GamePhase::from_lcu_str("InProgress"), GamePhase::InProgress);
        assert_eq!(
            GamePhase::from_lcu_str("PreEndOfGame"),
            GamePhase::EndOfGame
        );
        assert_eq!(GamePhase::from_lcu_str("EndOfGame"), GamePhase::EndOfGame);
    }

    #[test]
    fn falls_back_to_lobby_for_unknown_values() {
        assert_eq!(
            GamePhase::from_lcu_str("SomethingNewRiotAdded"),
            GamePhase::Lobby
        );
    }
}
