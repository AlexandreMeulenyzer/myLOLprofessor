export interface MatchHistoryEntry {
  matchId: string;
  queueId: number;
  patch: string;
  playedAt: string;
  durationSeconds: number;
  champion: string;
  teamPosition: string;
  win: boolean;
  statsJson: string;
}

export interface MatchParticipantStats {
  puuid: string;
  championName: string;
  championId: number;
  teamPosition: string;
  teamId: number;
  win: boolean;
  kills: number;
  deaths: number;
  assists: number;
  champLevel: number;
  goldEarned: number;
  totalMinionsKilled: number;
  neutralMinionsKilled: number;
  visionScore: number;
  summoner1Id: number;
  summoner2Id: number;
  item0: number;
  item1: number;
  item2: number;
  item3: number;
  item4: number;
  item5: number;
  item6: number;
}

export interface MatchDetailParticipant {
  puuid: string;
  gameName: string | null;
  tagLine: string | null;
  championId: number;
  championName: string;
  teamId: number;
  teamPosition: string;
  win: boolean;
  kills: number;
  deaths: number;
  assists: number;
  champLevel: number;
  goldEarned: number;
  totalMinionsKilled: number;
  neutralMinionsKilled: number;
  visionScore: number;
  totalDamageDealtToChampions: number;
  totalDamageTaken: number;
  wardsPlaced: number;
  wardsKilled: number;
  summoner1Id: number;
  summoner2Id: number;
  items: number[];
}

export interface MatchDetail {
  matchId: string;
  queueId: number;
  durationSeconds: number;
  participants: MatchDetailParticipant[];
}

export const QUEUE_NAMES: Record<number, string> = {
  420: "Classée Solo/Duo",
  440: "Classée Flex",
  400: "Normale (Draft)",
  430: "Normale (Blind)",
  450: "ARAM",
  700: "Clash",
};

export function queueName(queueId: number): string {
  return QUEUE_NAMES[queueId] ?? `File #${queueId}`;
}
