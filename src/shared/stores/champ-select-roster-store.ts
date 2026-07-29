import { create } from "zustand";
import { persist } from "zustand/middleware";

import type { ParticipantAnalysis } from "@/features/team-analysis/types";

interface ChampSelectRosterState {
  roster: ParticipantAnalysis[];
  capturedAt: number | null;
  setRoster: (roster: ParticipantAnalysis[]) => void;
}

/**
 * Dernier roster analyse en champion select (role, camp, champion par
 * joueur), persiste via localStorage pour survivre au passage de la
 * fenetre principale a la fenetre overlay — deux runtimes JS distincts qui
 * ne partagent pas de cache memoire (voir `useGamePhaseSync`). L'API LCU de
 * champion select n'est plus interrogeable une fois la partie lancee : ce
 * roster est la seule facon de retrouver "qui jouait quel role" pendant le
 * match (ex: identifier l'adversaire de lane dans l'overlay).
 */
export const useChampSelectRosterStore = create<ChampSelectRosterState>()(
  persist(
    (set) => ({
      roster: [],
      capturedAt: null,
      setRoster: (roster) => set({ roster, capturedAt: Date.now() }),
    }),
    { name: "wardstone-champ-select-roster" },
  ),
);
