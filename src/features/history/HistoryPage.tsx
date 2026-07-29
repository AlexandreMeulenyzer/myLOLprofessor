import { useMemo, useState } from "react";
import { Link } from "react-router-dom";

import { useChampionsById, useItems, useLatestPatchVersion } from "@/features/static-data/hooks";
import { Badge } from "@/shared/components/ui/Badge";
import { Button } from "@/shared/components/ui/Button";
import { Card } from "@/shared/components/ui/Card";
import { RemoteIcon } from "@/shared/components/ui/RemoteIcon";
import { championIconUrl, itemIconUrl } from "@/shared/lib/data-dragon-assets";
import { useActiveAccountStore } from "@/shared/stores/active-account-store";

import { ChampionPoolChart, LpProgressionChart } from "./HistoryCharts";
import { useMatchHistory, useSyncMatchHistory } from "./hooks";
import { MatchDetailModal } from "./MatchDetailModal";
import { queueName, type MatchHistoryEntry, type MatchParticipantStats } from "./types";

type ResultFilter = "all" | "win" | "loss";

function formatDuration(seconds: number): string {
  const minutes = Math.floor(seconds / 60);
  const remaining = seconds % 60;
  return `${minutes}:${remaining.toString().padStart(2, "0")}`;
}

function parseStats(match: MatchHistoryEntry): MatchParticipantStats | null {
  try {
    return JSON.parse(match.statsJson) as MatchParticipantStats;
  } catch {
    return null;
  }
}

function MatchRow({
  match,
  version,
  onSelect,
}: {
  match: MatchHistoryEntry;
  version: string | undefined;
  onSelect: (matchId: string) => void;
}) {
  const championsById = useChampionsById();
  const { data: items } = useItems();
  const stats = parseStats(match);
  const champion = championsById.get(match.champion);
  const csPerMin =
    stats && match.durationSeconds > 0
      ? (
          (stats.totalMinionsKilled + stats.neutralMinionsKilled) /
          (match.durationSeconds / 60)
        ).toFixed(1)
      : null;
  const itemIds = stats
    ? [stats.item0, stats.item1, stats.item2, stats.item3, stats.item4, stats.item5, stats.item6]
    : [];

  return (
    <button
      onClick={() => onSelect(match.matchId)}
      className={`flex w-full items-center justify-between gap-3 rounded-xl border-l-4 bg-[var(--color-surface-1)] px-4 py-3 text-left transition-colors hover:bg-[var(--color-surface-2)] ${
        match.win ? "border-l-[var(--color-win)]" : "border-l-[var(--color-loss)]"
      }`}
    >
      <div className="flex items-center gap-3">
        {version && (
          <RemoteIcon
            src={championIconUrl(version, match.champion)}
            alt={champion?.name ?? match.champion}
            className="h-10 w-10 shrink-0 rounded-full"
          />
        )}
        <div>
          <div className="flex items-center gap-2">
            <Badge tone={match.win ? "win" : "loss"}>{match.win ? "Victoire" : "Défaite"}</Badge>
            <span className="text-sm font-medium text-slate-100">
              {champion?.name ?? match.champion}
            </span>
          </div>
          <div className="mt-0.5 text-xs text-slate-500">
            {queueName(match.queueId)} · {match.teamPosition || "—"}
          </div>
          {stats && (
            <div className="mt-1 flex items-center gap-2 text-xs">
              <span className="font-mono text-slate-300">
                {stats.kills}/{stats.deaths}/{stats.assists}
              </span>
              {csPerMin && (
                <span className="text-slate-500">
                  {stats.totalMinionsKilled + stats.neutralMinionsKilled} CS ({csPerMin}/min)
                </span>
              )}
            </div>
          )}
          {version && stats && (
            <div className="mt-1 flex gap-0.5">
              {itemIds.map((itemId, index) => (
                <div
                  key={index}
                  className="h-5 w-5 rounded border border-[var(--color-border-subtle)] bg-black/20"
                >
                  {itemId > 0 && (
                    <RemoteIcon
                      src={itemIconUrl(version, itemId)}
                      alt={items?.[String(itemId)]?.name ?? `Objet #${itemId}`}
                      className="h-full w-full rounded"
                    />
                  )}
                </div>
              ))}
            </div>
          )}
        </div>
      </div>
      <div className="text-right text-xs text-slate-500">
        <div>{formatDuration(match.durationSeconds)}</div>
        <div>Patch {match.patch}</div>
      </div>
    </button>
  );
}

export function HistoryPage() {
  const activeAccount = useActiveAccountStore((state) => state.activeAccount);
  const { data: matches, isLoading } = useMatchHistory(activeAccount?.puuid);
  const syncHistory = useSyncMatchHistory(activeAccount?.puuid, activeAccount?.platform);
  const { data: version } = useLatestPatchVersion();

  const [championFilter, setChampionFilter] = useState("");
  const [resultFilter, setResultFilter] = useState<ResultFilter>("all");
  const [selectedMatchId, setSelectedMatchId] = useState<string | null>(null);

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
          <MatchRow
            key={match.matchId}
            match={match}
            version={version}
            onSelect={setSelectedMatchId}
          />
        ))}
      </div>

      {selectedMatchId && (
        <MatchDetailModal
          matchId={selectedMatchId}
          platform={activeAccount.platform}
          onClose={() => setSelectedMatchId(null)}
        />
      )}
    </div>
  );
}
