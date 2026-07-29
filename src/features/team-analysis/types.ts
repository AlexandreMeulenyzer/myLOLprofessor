export type TeamSide = "ally" | "enemy";

export interface ChampionMasterySummary {
  championId: number;
  championLevel: number;
  championPoints: number;
}

export interface ParticipantAnalysis {
  puuid: string;
  gameName: string;
  tagLine: string;
  championId: number;
  role: string;
  side: TeamSide;
  isLocalPlayer: boolean;
  tier: string | null;
  rank: string | null;
  leaguePoints: number | null;
  wins: number | null;
  losses: number | null;
  winratePercent: number | null;
  estimatedMmr: number | null;
  topChampionMasteries: ChampionMasterySummary[];
  hotStreak: boolean;
  veteran: boolean;
  freshBlood: boolean;
  topChampionMasterySharePercent: number | null;
  isPlayingTopMasteryChampion: boolean;
}

export const ROLE_LABELS: Record<string, string> = {
  TOP: "Top",
  JUNGLE: "Jungle",
  MIDDLE: "Mid",
  BOTTOM: "ADC",
  UTILITY: "Support",
};
