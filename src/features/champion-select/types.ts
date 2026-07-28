export interface LocalPlayerSelection {
  championId: number;
  role: string;
}

export interface ChampionRoleStats {
  championId: number;
  role: string;
  patch: string;
  games: number;
  wins: number;
  winratePercent: number;
  pickratePercent: number | null;
  banratePercent: number | null;
  avgKills: number;
  avgDeaths: number;
  avgAssists: number;
  avgGameDurationSeconds: number;
  avgCsPerMin: number;
  avgGoldPerMin: number;
  commonItems: [number, number][];
  commonSummonerSpells: [number, number][];
  commonKeystones: [number, number][];
}

export const ROLE_LABELS: Record<string, string> = {
  TOP: "Top",
  JUNGLE: "Jungle",
  MIDDLE: "Mid",
  BOTTOM: "ADC",
  UTILITY: "Support",
};
