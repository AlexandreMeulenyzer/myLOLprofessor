import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query";

import { isTauriRuntime } from "@/shared/lib/tauri-bridge";

import { getMatchHistory, syncMatchHistory } from "./api";

const DEFAULT_HISTORY_LIMIT = 20;

export function useMatchHistory(puuid: string | undefined, limit = DEFAULT_HISTORY_LIMIT) {
  return useQuery({
    queryKey: ["match-history", puuid, limit],
    queryFn: () => getMatchHistory(puuid as string, limit),
    enabled: isTauriRuntime() && !!puuid,
  });
}

export function useSyncMatchHistory(puuid: string | undefined, platform: string | undefined) {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: (count: number) => syncMatchHistory(puuid as string, platform as string, count),
    onSuccess: () => queryClient.invalidateQueries({ queryKey: ["match-history", puuid] }),
  });
}
