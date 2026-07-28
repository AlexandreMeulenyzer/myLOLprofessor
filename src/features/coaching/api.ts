import { invoke } from "@/shared/lib/tauri-bridge";

import type { CoachingReport } from "./types";

export async function getCoachingReport(
  puuid: string,
  matchId: string,
): Promise<CoachingReport | null> {
  return invoke<CoachingReport | null>("get_coaching_report", { puuid, matchId });
}
