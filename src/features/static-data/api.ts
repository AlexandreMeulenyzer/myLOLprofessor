import { invoke } from "@/shared/lib/tauri-bridge";

import type { ChampionDetail, ChampionSummary } from "./types";

export async function getLatestPatchVersion(): Promise<string> {
  return invoke<string>("get_latest_patch_version");
}

export async function getChampions(): Promise<ChampionSummary[]> {
  return invoke<ChampionSummary[]>("get_champions");
}

export async function getChampionDetail(championId: string): Promise<ChampionDetail> {
  return invoke<ChampionDetail>("get_champion_detail", { championId });
}
