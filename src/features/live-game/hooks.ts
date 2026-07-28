import { useQuery } from "@tanstack/react-query";

import { isTauriRuntime } from "@/shared/lib/tauri-bridge";
import { useGamePhaseStore } from "@/shared/stores/game-phase-store";

import { getLiveGameSnapshot } from "./api";

/** Interroge la Live Client Data API toutes les 2s pendant une partie. */
export function useLiveGameSnapshot() {
  const phase = useGamePhaseStore((state) => state.phase);
  const inGame = phase === "InProgress";

  return useQuery({
    queryKey: ["live-game", "snapshot"],
    queryFn: getLiveGameSnapshot,
    enabled: isTauriRuntime() && inGame,
    refetchInterval: inGame ? 2000 : false,
  });
}
