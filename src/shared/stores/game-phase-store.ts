import { create } from "zustand";

import type { GamePhase } from "@/shared/types/game-phase";

interface GamePhaseState {
  phase: GamePhase;
  since: number;
  setPhase: (phase: GamePhase) => void;
}

export const useGamePhaseStore = create<GamePhaseState>()((set) => ({
  phase: "ClientClosed",
  since: Date.now(),
  setPhase: (phase) => set({ phase, since: Date.now() }),
}));
