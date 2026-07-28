import { invoke } from "@/shared/lib/tauri-bridge";

import type { LiveGameSnapshot } from "./types";

export async function getLiveGameSnapshot(): Promise<LiveGameSnapshot | null> {
  return invoke<LiveGameSnapshot | null>("get_live_game_snapshot");
}
