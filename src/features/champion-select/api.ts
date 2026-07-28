import { invoke } from "@/shared/lib/tauri-bridge";

import type { ChampionRoleStats, LocalPlayerSelection } from "./types";

export async function getCurrentChampSelectSelection(): Promise<LocalPlayerSelection | null> {
  return invoke<LocalPlayerSelection | null>("get_current_champ_select_selection");
}

export async function getChampionRoleStats(
  championId: number,
  role: string,
  patch: string,
): Promise<ChampionRoleStats | null> {
  return invoke<ChampionRoleStats | null>("get_champion_role_stats", {
    championId,
    role,
    patch,
  });
}
