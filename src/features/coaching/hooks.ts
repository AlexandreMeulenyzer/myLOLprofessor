import { useQuery } from "@tanstack/react-query";

import { isTauriRuntime } from "@/shared/lib/tauri-bridge";

import { getCoachingReport } from "./api";

export function useCoachingReport(puuid: string | undefined, matchId: string | undefined) {
  return useQuery({
    queryKey: ["coaching", puuid, matchId],
    queryFn: () => getCoachingReport(puuid as string, matchId as string),
    enabled: isTauriRuntime() && !!puuid && !!matchId,
  });
}
