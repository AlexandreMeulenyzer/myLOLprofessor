import { invoke } from "@/shared/lib/tauri-bridge";

import type {
  ChampionDetail,
  ChampionSummary,
  ItemDetail,
  RuneTree,
  SummonerSpellDetail,
} from "./types";

export async function getLatestPatchVersion(): Promise<string> {
  return invoke<string>("get_latest_patch_version");
}

export async function getChampions(): Promise<ChampionSummary[]> {
  return invoke<ChampionSummary[]>("get_champions");
}

export async function getChampionDetail(championId: string): Promise<ChampionDetail> {
  return invoke<ChampionDetail>("get_champion_detail", { championId });
}

export async function getItems(): Promise<Record<string, ItemDetail>> {
  return invoke<Record<string, ItemDetail>>("get_items");
}

export async function getRunes(): Promise<RuneTree[]> {
  return invoke<RuneTree[]>("get_runes");
}

export async function getSummonerSpells(): Promise<Record<string, SummonerSpellDetail>> {
  return invoke<Record<string, SummonerSpellDetail>>("get_summoner_spells");
}
