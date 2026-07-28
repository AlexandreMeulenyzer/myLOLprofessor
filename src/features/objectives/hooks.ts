import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query";

import { isTauriRuntime } from "@/shared/lib/tauri-bridge";

import { createObjective, deleteObjective, listObjectivesWithProgress } from "./api";
import type { ObjectiveKind } from "./types";

export function useObjectives(puuid: string | undefined, platform: string | undefined) {
  return useQuery({
    queryKey: ["objectives", puuid, platform],
    queryFn: () => listObjectivesWithProgress(puuid as string, platform as string),
    enabled: isTauriRuntime() && !!puuid && !!platform,
  });
}

export function useCreateObjective(puuid: string | undefined) {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: (kind: ObjectiveKind) => createObjective(puuid as string, kind),
    onSuccess: () => queryClient.invalidateQueries({ queryKey: ["objectives", puuid] }),
  });
}

export function useDeleteObjective(puuid: string | undefined) {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: (id: number) => deleteObjective(id, puuid as string),
    onSuccess: () => queryClient.invalidateQueries({ queryKey: ["objectives", puuid] }),
  });
}
