import { invoke } from "@/shared/lib/tauri-bridge";

import type { ObjectiveKind, ObjectiveWithProgress } from "./types";

export async function createObjective(puuid: string, kind: ObjectiveKind): Promise<number> {
  return invoke<number>("create_objective", { puuid, kind });
}

export async function deleteObjective(id: number, puuid: string): Promise<void> {
  return invoke<void>("delete_objective", { id, puuid });
}

export async function listObjectivesWithProgress(
  puuid: string,
  platform: string,
): Promise<ObjectiveWithProgress[]> {
  return invoke<ObjectiveWithProgress[]>("list_objectives_with_progress", { puuid, platform });
}
