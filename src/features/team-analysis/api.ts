import { invoke } from "@/shared/lib/tauri-bridge";

import type { ParticipantAnalysis } from "./types";

export async function getChampSelectTeamAnalysis(platform: string): Promise<ParticipantAnalysis[]> {
  return invoke<ParticipantAnalysis[]>("get_champ_select_team_analysis", { platform });
}
