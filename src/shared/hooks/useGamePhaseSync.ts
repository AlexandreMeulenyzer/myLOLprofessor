import { useEffect } from "react";

import { invoke, isTauriRuntime, listen } from "@/shared/lib/tauri-bridge";
import { useGamePhaseStore } from "@/shared/stores/game-phase-store";
import type { GamePhase } from "@/shared/types/game-phase";

/**
 * Synchronise le store `game-phase` avec le backend Rust : recupere la
 * phase courante au montage, puis ecoute l'evenement
 * `game-phase-changed` emis par le connecteur LCU (src-tauri/src/infrastructure/lcu).
 * A monter une fois par fenetre (fenetre principale et fenetre overlay
 * sont deux runtimes JS distincts).
 */
export function useGamePhaseSync(): void {
  const setPhase = useGamePhaseStore((state) => state.setPhase);

  useEffect(() => {
    if (!isTauriRuntime()) {
      return;
    }

    let cancelled = false;

    invoke<GamePhase>("get_game_phase")
      .then((phase) => {
        if (!cancelled) setPhase(phase);
      })
      .catch(() => {
        // Le backend n'est pas encore pret (tres tot au demarrage) ; le
        // premier evenement `game-phase-changed` rattrapera l'etat.
      });

    const unlistenPromise = listen<GamePhase>("game-phase-changed", (event) => {
      setPhase(event.payload);
    });

    return () => {
      cancelled = true;
      unlistenPromise.then((unlisten) => unlisten());
    };
  }, [setPhase]);
}
