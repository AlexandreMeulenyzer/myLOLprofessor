import { invoke } from "@/shared/lib/tauri-bridge";

import type { MatchDetail, MatchHistoryEntry } from "./types";

export async function syncMatchHistory(
  puuid: string,
  platform: string,
  count: number,
): Promise<number> {
  return invoke<number>("sync_match_history", { puuid, platform, count });
}

export async function getMatchHistory(puuid: string, limit: number): Promise<MatchHistoryEntry[]> {
  return invoke<MatchHistoryEntry[]>("get_match_history", { puuid, limit });
}

export async function getMatchDetail(matchId: string, platform: string): Promise<MatchDetail> {
  return invoke<MatchDetail>("get_match_detail", { matchId, platform });
}
