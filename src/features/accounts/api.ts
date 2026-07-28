import { invoke } from "@/shared/lib/tauri-bridge";

import type { AccountRecord } from "./types";

export async function hasRiotApiKey(): Promise<boolean> {
  return invoke<boolean>("has_riot_api_key");
}

export async function saveRiotApiKey(apiKey: string): Promise<void> {
  return invoke<void>("save_riot_api_key", { apiKey });
}

export async function deleteRiotApiKey(): Promise<void> {
  return invoke<void>("delete_riot_api_key");
}

export async function listAccounts(): Promise<AccountRecord[]> {
  return invoke<AccountRecord[]>("list_accounts");
}

export async function linkAccount(
  gameName: string,
  tagLine: string,
  platform: string,
): Promise<AccountRecord> {
  return invoke<AccountRecord>("link_account", { gameName, tagLine, platform });
}

export async function removeAccount(puuid: string): Promise<void> {
  return invoke<void>("remove_account", { puuid });
}

export async function setPrimaryAccount(puuid: string): Promise<void> {
  return invoke<void>("set_primary_account", { puuid });
}
