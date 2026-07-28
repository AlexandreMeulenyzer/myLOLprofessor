import { invoke } from "@/shared/lib/tauri-bridge";

import type { LeagueSnapshotRecord, ProfileSummary } from "./types";

export async function getProfile(puuid: string, platform: string): Promise<ProfileSummary> {
  return invoke<ProfileSummary>("get_profile", { puuid, platform });
}

export async function getLpHistory(
  puuid: string,
  queueType: string,
  limit: number,
): Promise<LeagueSnapshotRecord[]> {
  return invoke<LeagueSnapshotRecord[]>("get_lp_history", { puuid, queueType, limit });
}
