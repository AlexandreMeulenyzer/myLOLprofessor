/**
 * Miroir de `domain::game_state::GamePhase` (src-tauri). Toute modification
 * cote Rust doit etre repercutee ici.
 */
export type GamePhase =
  | "ClientClosed"
  | "ClientLaunched"
  | "Lobby"
  | "Matchmaking"
  | "ReadyCheck"
  | "ChampSelect"
  | "GameStart"
  | "InProgress"
  | "WaitingForStats"
  | "EndOfGame";

export const GAME_PHASE_LABELS: Record<GamePhase, string> = {
  ClientClosed: "Client fermé",
  ClientLaunched: "Client lancé",
  Lobby: "Lobby",
  Matchmaking: "Recherche de partie",
  ReadyCheck: "Partie trouvée",
  ChampSelect: "Sélection des champions",
  GameStart: "Chargement de la partie",
  InProgress: "Partie en cours",
  WaitingForStats: "Fin de partie",
  EndOfGame: "Résultats",
};
