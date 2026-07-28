import { invoke } from "@/shared/lib/tauri-bridge";

import type { ProfileSummary } from "./types";

export async function getProfile(puuid: string, platform: string): Promise<ProfileSummary> {
  return invoke<ProfileSummary>("get_profile", { puuid, platform });
}
