export interface ObjectiveTimers {
  nextDragonSeconds: number;
  nextBaronSeconds: number;
  heraldAvailable: boolean;
}

export interface LivePlayerSnapshot {
  summonerName: string;
  championName: string;
  team: string;
  level: number;
  kills: number;
  deaths: number;
  assists: number;
  creepScore: number;
  itemCount: number;
  isDead: boolean;
  respawnTimerSeconds: number;
}

export interface LiveGameSnapshot {
  gameTimeSeconds: number;
  activePlayerName: string;
  activePlayerGold: number;
  activePlayerLevel: number;
  objectiveTimers: ObjectiveTimers;
  contextualTip: string;
  players: LivePlayerSnapshot[];
}

export function formatTimer(seconds: number): string {
  if (seconds <= 0) return "Disponible";
  const minutes = Math.floor(seconds / 60);
  const remaining = Math.floor(seconds % 60);
  return `${minutes}:${remaining.toString().padStart(2, "0")}`;
}

export function formatGameClock(seconds: number): string {
  const minutes = Math.floor(seconds / 60);
  const remaining = Math.floor(seconds % 60);
  return `${minutes}:${remaining.toString().padStart(2, "0")}`;
}
