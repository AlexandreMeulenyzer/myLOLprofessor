import { Badge } from "@/shared/components/ui/Badge";
import { useGamePhaseStore } from "@/shared/stores/game-phase-store";
import { GAME_PHASE_LABELS } from "@/shared/types/game-phase";

const ACTIVE_PHASE_TONE = new Set(["ChampSelect", "InProgress", "ReadyCheck"]);

export function Topbar() {
  const phase = useGamePhaseStore((state) => state.phase);

  return (
    <header className="flex h-14 shrink-0 items-center justify-between border-b border-[var(--color-border-subtle)] px-5">
      <div className="text-sm text-slate-400">
        Compagnon League of Legends — données Riot API officielles
      </div>
      <Badge tone={ACTIVE_PHASE_TONE.has(phase) ? "accent" : "neutral"}>
        {GAME_PHASE_LABELS[phase]}
      </Badge>
    </header>
  );
}
