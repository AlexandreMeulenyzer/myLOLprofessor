import { invoke } from "@/shared/lib/tauri-bridge";

import type { ResolvedSummoner } from "./types";

export async function resolveSummoner(
  gameName: string,
  tagLine: string,
  platform: string,
): Promise<ResolvedSummoner> {
  return invoke<ResolvedSummoner>("resolve_summoner", { gameName, tagLine, platform });
}
