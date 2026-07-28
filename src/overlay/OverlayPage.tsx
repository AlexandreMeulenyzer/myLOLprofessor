import { useLiveGameSnapshot } from "@/features/live-game/hooks";
import { formatGameClock, formatTimer } from "@/features/live-game/types";
import { useGamePhaseSync } from "@/shared/hooks/useGamePhaseSync";
import { invoke } from "@/shared/lib/tauri-bridge";
import { useGamePhaseStore } from "@/shared/stores/game-phase-store";
import { useOverlaySettingsStore } from "@/shared/stores/overlay-settings-store";
import { GAME_PHASE_LABELS } from "@/shared/types/game-phase";

function closeOverlay() {
  void invoke("toggle_overlay_window");
}

export function OverlayPage() {
  useGamePhaseSync();
  const phase = useGamePhaseStore((state) => state.phase);
  const { data: snapshot } = useLiveGameSnapshot();
  const showGoldAndLevel = useOverlaySettingsStore((state) => state.showGoldAndLevel);
  const showObjectiveTimers = useOverlaySettingsStore((state) => state.showObjectiveTimers);
  const showContextualTip = useOverlaySettingsStore((state) => state.showContextualTip);

  return (
    <div
      data-tauri-drag-region
      className="flex h-screen w-screen flex-col overflow-hidden overlay-transparent p-2"
    >
      <div className="glass-panel flex flex-1 flex-col gap-2 rounded-2xl p-3 text-slate-100 shadow-lg">
        <div data-tauri-drag-region className="flex items-center justify-between">
          <span className="text-xs font-semibold uppercase tracking-wide text-[var(--color-accent-400)]">
            Wardstone
          </span>
          <button
            onClick={closeOverlay}
            className="rounded-md px-1.5 text-xs text-slate-400 hover:bg-white/10 hover:text-slate-100"
            aria-label="Fermer l'overlay"
          >
            ✕
          </button>
        </div>

        {!snapshot ? (
          <div className="flex flex-1 items-center justify-center text-sm text-slate-400">
            {GAME_PHASE_LABELS[phase]}
          </div>
        ) : (
          <>
            <div className="flex items-center justify-between text-sm">
              <span className="font-mono text-slate-200">
                ⏱ {formatGameClock(snapshot.gameTimeSeconds)}
              </span>
              {showGoldAndLevel && (
                <span className="text-slate-300">
                  💰 {Math.round(snapshot.activePlayerGold)} · Nv.{snapshot.activePlayerLevel}
                </span>
              )}
            </div>

            {showObjectiveTimers && (
              <div className="grid grid-cols-3 gap-2 text-center text-xs">
                <div className="rounded-lg bg-white/5 px-2 py-1.5">
                  <div className="text-slate-400">Dragon</div>
                  <div className="font-mono text-slate-100">
                    {formatTimer(snapshot.objectiveTimers.nextDragonSeconds)}
                  </div>
                </div>
                <div className="rounded-lg bg-white/5 px-2 py-1.5">
                  <div className="text-slate-400">Baron</div>
                  <div className="font-mono text-slate-100">
                    {formatTimer(snapshot.objectiveTimers.nextBaronSeconds)}
                  </div>
                </div>
                <div className="rounded-lg bg-white/5 px-2 py-1.5">
                  <div className="text-slate-400">Héraut</div>
                  <div className="font-mono text-slate-100">
                    {snapshot.objectiveTimers.heraldAvailable ? "Disponible" : "—"}
                  </div>
                </div>
              </div>
            )}

            {showContextualTip && (
              <p className="rounded-lg bg-[var(--color-accent-500)]/10 px-2 py-1.5 text-xs text-[var(--color-accent-400)]">
                {snapshot.contextualTip}
              </p>
            )}
          </>
        )}
      </div>
    </div>
  );
}
