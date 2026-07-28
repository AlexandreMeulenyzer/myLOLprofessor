import { useQuery } from "@tanstack/react-query";

import { isTauriRuntime } from "@/shared/lib/tauri-bridge";

import { getProfile } from "./api";

export function useProfile(puuid: string | undefined, platform: string | undefined) {
  return useQuery({
    queryKey: ["profile", puuid, platform],
    queryFn: () => getProfile(puuid as string, platform as string),
    enabled: isTauriRuntime() && !!puuid && !!platform,
    staleTime: 60_000,
  });
}
