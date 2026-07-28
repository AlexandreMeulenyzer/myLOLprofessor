export interface LeagueEntrySummary {
  queueType: string;
  tier: string;
  rank: string;
  leaguePoints: number;
  wins: number;
  losses: number;
  winratePercent: number;
  estimatedMmr: number | null;
}

export interface ChampionMasterySummary {
  championId: number;
  championLevel: number;
  championPoints: number;
}

export interface ProfileSummary {
  puuid: string;
  profileIconId: number;
  summonerLevel: number;
  leagueEntries: LeagueEntrySummary[];
  topChampions: ChampionMasterySummary[];
}

export const QUEUE_LABELS: Record<string, string> = {
  RANKED_SOLO_5x5: "Solo/Duo",
  RANKED_FLEX_SR: "Flex",
};
