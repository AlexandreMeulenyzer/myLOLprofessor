import { invoke } from "@/shared/lib/tauri-bridge";

import type { MatchHistoryEntry } from "./types";

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
