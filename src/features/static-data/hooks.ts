import { useQuery } from "@tanstack/react-query";
import { useMemo } from "react";

import { isTauriRuntime } from "@/shared/lib/tauri-bridge";

import { getChampionDetail, getChampions, getLatestPatchVersion } from "./api";
import type { ChampionSummary } from "./types";

export function useLatestPatchVersion() {
  return useQuery({
    queryKey: ["static-data", "version"],
    queryFn: getLatestPatchVersion,
    enabled: isTauriRuntime(),
    staleTime: 60 * 60_000,
  });
}

export function useChampions() {
  return useQuery({
    queryKey: ["static-data", "champions"],
    queryFn: getChampions,
    enabled: isTauriRuntime(),
    staleTime: 60 * 60_000,
  });
}

export function useChampionDetail(championId: string | undefined) {
  return useQuery({
    queryKey: ["static-data", "champion", championId],
    queryFn: () => getChampionDetail(championId as string),
    enabled: isTauriRuntime() && !!championId,
    staleTime: 60 * 60_000,
  });
}

/** Map championId numerique (match-v5, champion-mastery-v4) -> ChampionSummary. */
export function useChampionsByKey() {
  const { data: champions } = useChampions();
  return useMemo(() => {
    const map = new Map<string, ChampionSummary>();
    champions?.forEach((champion) => map.set(champion.key, champion));
    return map;
  }, [champions]);
}
