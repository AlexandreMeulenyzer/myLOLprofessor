import { useMemo, useState } from "react";

import { useChampionRoleStats } from "@/features/champion-select/hooks";
import { ROLE_LABELS } from "@/features/champion-select/types";
import { useChampions, useLatestPatchVersion } from "@/features/static-data/hooks";
import { Badge } from "@/shared/components/ui/Badge";
import { Card } from "@/shared/components/ui/Card";

function ChampionSlot({ label }: { label: string }) {
  const { data: champions } = useChampions();
  const { data: patch } = useLatestPatchVersion();
  const [championKey, setChampionKey] = useState<string>("");
  const [role, setRole] = useState<string>("MIDDLE");

  const sortedChampions = useMemo(
    () => [...(champions ?? [])].sort((a, b) => a.name.localeCompare(b.name)),
    [champions],
  );

  const { data: stats, isLoading } = useChampionRoleStats(
    championKey ? Number(championKey) : undefined,
    role,
    patch,
  );

  const championName = sortedChampions.find((c) => c.key === championKey)?.name;

  return (
    <Card heading={label} glass className="flex-1">
      <div className="mb-3 flex flex-wrap gap-1.5">
        <select
          value={championKey}
          onChange={(e) => setChampionKey(e.target.value)}
          className="min-w-0 flex-1 rounded-xl border border-[var(--color-border-subtle)] bg-[var(--color-surface-2)] px-2 py-1.5 text-sm text-slate-100"
        >
          <option value="">Choisir un champion…</option>
          {sortedChampions.map((champion) => (
            <option key={champion.key} value={champion.key}>
              {champion.name}
            </option>
          ))}
        </select>
        <select
          value={role}
          onChange={(e) => setRole(e.target.value)}
          className="rounded-xl border border-[var(--color-border-subtle)] bg-[var(--color-surface-2)] px-2 py-1.5 text-xs text-slate-100"
        >
          {Object.entries(ROLE_LABELS).map(([value, roleLabel]) => (
            <option key={value} value={value}>
              {roleLabel}
            </option>
          ))}
        </select>
      </div>

      {!championKey && (
        <p className="text-xs text-slate-500">Sélectionnez un champion à comparer.</p>
      )}

      {championKey && isLoading && <p className="text-xs text-slate-500">Chargement…</p>}

      {championKey && !isLoading && !stats && (
        <p className="text-xs text-slate-500">
          Pas encore assez de données locales pour {championName} en {ROLE_LABELS[role] ?? role}.
        </p>
      )}

      {stats && (
        <div className="flex flex-col gap-2 text-sm">
          <div className="grid grid-cols-2 gap-2 sm:grid-cols-3">
            <div className="rounded-xl border border-[var(--color-border-subtle)] px-2 py-1.5 text-center">
              <div className="text-lg font-semibold text-slate-100">{stats.winratePercent}%</div>
              <div className="text-xs text-slate-500">Winrate</div>
            </div>
            <div className="rounded-xl border border-[var(--color-border-subtle)] px-2 py-1.5 text-center">
              <div className="text-lg font-semibold text-slate-100">
                {stats.pickratePercent !== null ? `${stats.pickratePercent}%` : "—"}
              </div>
              <div className="text-xs text-slate-500">Pickrate</div>
            </div>
            <div className="rounded-xl border border-[var(--color-border-subtle)] px-2 py-1.5 text-center">
              <div className="text-lg font-semibold text-slate-100">
                {stats.banratePercent !== null ? `${stats.banratePercent}%` : "—"}
              </div>
              <div className="text-xs text-slate-500">Banrate</div>
            </div>
          </div>
          <div className="text-xs text-slate-400">
            KDA moyen : {stats.avgKills} / {stats.avgDeaths} / {stats.avgAssists} — CS/min{" "}
            {stats.avgCsPerMin ?? "—"} — Or/min {stats.avgGoldPerMin ?? "—"}
          </div>
          <div className="text-xs text-slate-500">
            Basé sur {stats.games} partie(s) synchronisée(s) — patch {stats.patch}
          </div>
          {stats.commonItems.length > 0 && (
            <div className="flex flex-wrap gap-1.5">
              {stats.commonItems.slice(0, 4).map(([itemId, count]) => (
                <Badge key={itemId} tone="neutral">
                  Objet #{itemId} ({count})
                </Badge>
              ))}
            </div>
          )}
        </div>
      )}
    </Card>
  );
}

export function ChampionComparison() {
  return (
    <div className="flex flex-col gap-4 sm:flex-row">
      <ChampionSlot label="Champion A" />
      <ChampionSlot label="Champion B" />
    </div>
  );
}
