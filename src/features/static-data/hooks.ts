import { useQuery } from "@tanstack/react-query";
import { useMemo } from "react";

import { isTauriRuntime } from "@/shared/lib/tauri-bridge";

import {
  getChampionDetail,
  getChampions,
  getItems,
  getLatestPatchVersion,
  getRunes,
  getSummonerSpells,
} from "./api";
import type { ChampionSummary, Rune, SummonerSpellDetail } from "./types";

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

export function useItems() {
  return useQuery({
    queryKey: ["static-data", "items"],
    queryFn: getItems,
    enabled: isTauriRuntime(),
    staleTime: 60 * 60_000,
  });
}

export function useRunes() {
  return useQuery({
    queryKey: ["static-data", "runes"],
    queryFn: getRunes,
    enabled: isTauriRuntime(),
    staleTime: 60 * 60_000,
  });
}

/** Map runeId numerique (match-v5 perks) -> Rune (nom, icone, description). */
export function useRunesById() {
  const { data: trees } = useRunes();
  return useMemo(() => {
    const map = new Map<number, Rune>();
    trees?.forEach((tree) => {
      tree.slots.forEach((slot) => {
        slot.runes.forEach((rune) => map.set(rune.id, rune));
      });
    });
    return map;
  }, [trees]);
}

export function useSummonerSpells() {
  return useQuery({
    queryKey: ["static-data", "summoner-spells"],
    queryFn: getSummonerSpells,
    enabled: isTauriRuntime(),
    staleTime: 60 * 60_000,
  });
}

/** Map identifiant numerique (summoner1Id/summoner2Id) -> SummonerSpellDetail. */
export function useSummonerSpellsByKey() {
  const { data: spells } = useSummonerSpells();
  return useMemo(() => {
    const map = new Map<string, SummonerSpellDetail>();
    Object.values(spells ?? {}).forEach((spell) => map.set(spell.key, spell));
    return map;
  }, [spells]);
}
