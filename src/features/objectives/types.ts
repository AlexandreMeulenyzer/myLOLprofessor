export type ObjectiveKind =
  | { type: "reachRank"; queueType: string; tier: string; rank: string }
  | { type: "winrateTarget"; queueType: string; percent: number; minGames: number }
  | { type: "gamesPlayed"; count: number };

export interface ObjectiveProgress {
  percent: number;
  achieved: boolean;
  summary: string;
}

export interface ObjectiveWithProgress {
  id: number;
  kind: ObjectiveKind;
  createdAt: string;
  achievedAt: string | null;
  progress: ObjectiveProgress;
}

export const TIERS = [
  "IRON",
  "BRONZE",
  "SILVER",
  "GOLD",
  "PLATINUM",
  "EMERALD",
  "DIAMOND",
  "MASTER",
  "GRANDMASTER",
  "CHALLENGER",
] as const;

export const DIVISIONS = ["IV", "III", "II", "I"] as const;

export function describeObjective(kind: ObjectiveKind): string {
  switch (kind.type) {
    case "reachRank":
      return `Atteindre ${kind.tier} ${kind.rank}`;
    case "winrateTarget":
      return `${kind.percent}% de winrate (min. ${kind.minGames} parties)`;
    case "gamesPlayed":
      return `Jouer ${kind.count} parties`;
  }
}
