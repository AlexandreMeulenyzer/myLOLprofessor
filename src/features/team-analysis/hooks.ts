import { useQuery } from "@tanstack/react-query";

import { isTauriRuntime } from "@/shared/lib/tauri-bridge";
import { useGamePhaseStore } from "@/shared/stores/game-phase-store";

import { getChampSelectTeamAnalysis } from "./api";

/**
 * Analyse d'equipe : appelee une fois a l'entree en champion select (pas de
 * polling continu — chaque appel declenche jusqu'a ~30 requetes Riot API
 * pour les 10 joueurs, un rafraichissement periodique gaspillerait le
 * budget de rate-limit pour peu de benefice sur une session de quelques
 * dizaines de secondes).
 */
export function useChampSelectTeamAnalysis(platform: string | undefined) {
  const phase = useGamePhaseStore((state) => state.phase);
  const inChampSelect = phase === "ChampSelect";

  return useQuery({
    queryKey: ["team-analysis", platform],
    queryFn: () => getChampSelectTeamAnalysis(platform as string),
    enabled: isTauriRuntime() && inChampSelect && !!platform,
    staleTime: Infinity,
    retry: 1,
  });
}
