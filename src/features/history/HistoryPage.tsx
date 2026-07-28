import { useMemo, useState } from "react";
import { Link } from "react-router-dom";

import { Badge } from "@/shared/components/ui/Badge";
import { Button } from "@/shared/components/ui/Button";
import { Card } from "@/shared/components/ui/Card";
import { useActiveAccountStore } from "@/shared/stores/active-account-store";

import { ChampionPoolChart, LpProgressionChart } from "./HistoryCharts";
import { useMatchHistory, useSyncMatchHistory } from "./hooks";
import { queueName } from "./types";

type ResultFilter = "all" | "win" | "loss";

function formatDuration(seconds: number): string {
  const minutes = Math.floor(seconds / 60);
  const remaining = seconds % 60;
  return `${minutes}:${remaining.toString().padStart(2, "0")}`;
}

export function HistoryPage() {
  const activeAccount = useActiveAccountStore((state) => state.activeAccount);
  const { data: matches, isLoading } = useMatchHistory(activeAccount?.puuid);
  const syncHistory = useSyncMatchHistory(activeAccount?.puuid, activeAccount?.platform);

  const [championFilter, setChampionFilter] = useState("");
  const [resultFilter, setResultFilter] = useState<ResultFilter>("all");

  const filtered = useMemo(() => {
    if (!matches) return [];
    return matches.filter((match) => {
      if (resultFilter === "win" && !match.win) return false;
      if (resultFilter === "loss" && match.win) return false;
      if (championFilter && !match.champion.toLowerCase().includes(championFilter.toLowerCase())) {
        return false;
      }
      return true;
    });
  }, [matches, championFilter, resultFilter]);

  if (!activeAccount) {
    return (
      <div className="mx-auto max-w-lg py-16 text-center">
        <p className="text-slate-400">Aucun compte actif. Liez un compte pour voir l'historique.</p>
        <Link
          to="/accounts"
          className="mt-3 inline-block text-[var(--color-accent-400)] hover:underline"
        >
          Aller à la page Comptes →
        </Link>
      </div>
    );
  }

  return (
    <div className="mx-auto flex max-w-3xl flex-col gap-4 py-8">
      <div className="flex items-center justify-between">
        <h1 className="text-2xl font-semibold text-slate-100">Historique</h1>
        <Button
          size="sm"
          variant="secondary"
          disabled={syncHistory.isPending}
          onClick={() => syncHistory.mutate(20)}
        >
          {syncHistory.isPending ? "Synchronisation…" : "Synchroniser"}
        </Button>
      </div>

      {syncHistory.isSuccess && (
        <p className="text-xs text-[var(--color-accent-400)]">
          {syncHistory.data} nouvelle(s) partie(s) synchronisée(s).
        </p>
      )}
      {syncHistory.isError && (
        <p className="text-xs text-[var(--color-loss)]">
          {syncHistory.error instanceof Error
            ? syncHistory.error.message
            : String(syncHistory.error)}
        </p>
      )}

      <div className="grid gap-4 sm:grid-cols-2">
        <LpProgressionChart puuid={activeAccount.puuid} />
        <ChampionPoolChart matches={matches ?? []} />
      </div>

      <Card glass className="flex flex-wrap items-center gap-2">
        <input
          value={championFilter}
          onChange={(event) => setChampionFilter(event.target.value)}
          placeholder="Filtrer par champion…"
          className="flex-1 rounded-xl border border-[var(--color-border-subtle)] bg-[var(--color-surface-2)] px-3 py-1.5 text-sm text-slate-100 outline-none focus:border-[var(--color-accent-500)]"
        />
        <div className="flex gap-1">
          {(["all", "win", "loss"] as const).map((value) => (
            <button
              key={value}
              onClick={() => setResultFilter(value)}
              className={`rounded-lg px-3 py-1.5 text-xs font-medium ${
                resultFilter === value
                  ? "bg-[var(--color-accent-500)]/15 text-[var(--color-accent-400)]"
                  : "text-slate-400 hover:bg-[var(--color-surface-2)]"
              }`}
            >
              {value === "all" ? "Toutes" : value === "win" ? "Victoires" : "Défaites"}
            </button>
          ))}
        </div>
      </Card>

      {isLoading && <p className="text-sm text-slate-400">Chargement…</p>}

      {!isLoading && filtered.length === 0 && (
        <p className="py-8 text-center text-sm text-slate-400">
          Aucune partie synchronisée. Cliquez sur « Synchroniser » pour récupérer votre historique
          récent depuis l'API Riot.
        </p>
      )}

      <div className="flex flex-col gap-2">
        {filtered.map((match) => (
          <div
            key={match.matchId}
            className={`flex items-center justify-between rounded-xl border-l-4 bg-[var(--color-surface-1)] px-4 py-3 ${
              match.win ? "border-l-[var(--color-win)]" : "border-l-[var(--color-loss)]"
            }`}
          >
            <div className="flex items-center gap-3">
              <Badge tone={match.win ? "win" : "loss"}>{match.win ? "Victoire" : "Défaite"}</Badge>
              <div>
                <div className="text-sm font-medium text-slate-100">{match.champion}</div>
                <div className="text-xs text-slate-500">
                  {queueName(match.queueId)} · {match.teamPosition || "—"}
                </div>
              </div>
            </div>
            <div className="text-right text-xs text-slate-500">
              <div>{formatDuration(match.durationSeconds)}</div>
              <div>Patch {match.patch}</div>
            </div>
          </div>
        ))}
      </div>
    </div>
  );
}
