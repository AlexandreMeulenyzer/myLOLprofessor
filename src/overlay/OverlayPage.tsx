import { useGamePhaseStore } from "@/shared/stores/game-phase-store";
import { GAME_PHASE_LABELS } from "@/shared/types/game-phase";

/**
 * Contenu affiche dans la fenetre overlay (transparente, always-on-top).
 * L'implementation complete (timers d'objectifs, or estime, conseils) arrive
 * avec l'Epic 5 — voir docs/ROADMAP.md.
 */
export function OverlayPage() {
  const phase = useGamePhaseStore((state) => state.phase);

  return (
    <div className="flex h-screen w-screen items-start justify-center overflow-hidden overlay-transparent p-2">
      <div className="glass-panel rounded-2xl px-4 py-2 text-sm text-slate-100 shadow-lg">
        Wardstone — {GAME_PHASE_LABELS[phase]}
      </div>
    </div>
  );
}
