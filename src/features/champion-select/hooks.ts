import { useQuery } from "@tanstack/react-query";

import { isTauriRuntime } from "@/shared/lib/tauri-bridge";
import { useGamePhaseStore } from "@/shared/stores/game-phase-store";

import { getChampionRoleStats, getCurrentChampSelectSelection } from "./api";

/** Interroge le LCU toutes les 2s tant que le joueur est en champion select. */
export function useChampSelectSelection() {
  const phase = useGamePhaseStore((state) => state.phase);
  const inChampSelect = phase === "ChampSelect";

  return useQuery({
    queryKey: ["champ-select", "selection"],
    queryFn: getCurrentChampSelectSelection,
    enabled: isTauriRuntime() && inChampSelect,
    refetchInterval: inChampSelect ? 2000 : false,
  });
}

export function useChampionRoleStats(
  championId: number | undefined,
  role: string | undefined,
  patch: string | undefined,
) {
  return useQuery({
    queryKey: ["champ-select", "stats", championId, role, patch],
    queryFn: () => getChampionRoleStats(championId as number, role as string, patch as string),
    enabled: isTauriRuntime() && !!championId && !!role && !!patch,
  });
}
