import { useState } from "react";
import { Link } from "react-router-dom";

import { Badge } from "@/shared/components/ui/Badge";
import { Button } from "@/shared/components/ui/Button";
import { Card } from "@/shared/components/ui/Card";
import { useActiveAccountStore } from "@/shared/stores/active-account-store";

import { useCreateObjective, useDeleteObjective, useObjectives } from "./hooks";
import { describeObjective, DIVISIONS, TIERS, type ObjectiveKind } from "./types";

const NO_DIVISION_TIERS = new Set(["MASTER", "GRANDMASTER", "CHALLENGER"]);

function NewObjectiveForm({ puuid }: { puuid: string }) {
  const [kind, setKind] = useState<ObjectiveKind["type"]>("reachRank");
  const [tier, setTier] = useState<(typeof TIERS)[number]>("GOLD");
  const [division, setDivision] = useState<(typeof DIVISIONS)[number]>("IV");
  const [percent, setPercent] = useState(60);
  const [minGames, setMinGames] = useState(20);
  const [count, setCount] = useState(50);
  const createObjective = useCreateObjective(puuid);

  function submit() {
    let objective: ObjectiveKind;
    if (kind === "reachRank") {
      objective = {
        type: "reachRank",
        queueType: "RANKED_SOLO_5x5",
        tier,
        rank: NO_DIVISION_TIERS.has(tier) ? "I" : division,
      };
    } else if (kind === "winrateTarget") {
      objective = { type: "winrateTarget", queueType: "RANKED_SOLO_5x5", percent, minGames };
    } else {
      objective = { type: "gamesPlayed", count };
    }
    createObjective.mutate(objective);
  }

  return (
    <Card heading="Nouvel objectif" glass className="w-full">
      <div className="flex flex-col gap-3">
        <div className="flex gap-1">
          {(
            [
              ["reachRank", "Rang cible"],
              ["winrateTarget", "Winrate cible"],
              ["gamesPlayed", "Nombre de parties"],
            ] as const
          ).map(([value, label]) => (
            <button
              key={value}
              onClick={() => setKind(value)}
              className={`rounded-lg px-3 py-1.5 text-xs font-medium ${
                kind === value
                  ? "bg-[var(--color-accent-500)]/15 text-[var(--color-accent-400)]"
                  : "text-slate-400 hover:bg-[var(--color-surface-2)]"
              }`}
            >
              {label}
            </button>
          ))}
        </div>

        {kind === "reachRank" && (
          <div className="flex gap-2">
            <select
              value={tier}
              onChange={(e) => setTier(e.target.value as (typeof TIERS)[number])}
              className="rounded-xl border border-[var(--color-border-subtle)] bg-[var(--color-surface-2)] px-3 py-2 text-sm text-slate-100"
            >
              {TIERS.map((t) => (
                <option key={t} value={t}>
                  {t}
                </option>
              ))}
            </select>
            {!NO_DIVISION_TIERS.has(tier) && (
              <select
                value={division}
                onChange={(e) => setDivision(e.target.value as (typeof DIVISIONS)[number])}
                className="rounded-xl border border-[var(--color-border-subtle)] bg-[var(--color-surface-2)] px-3 py-2 text-sm text-slate-100"
              >
                {DIVISIONS.map((d) => (
                  <option key={d} value={d}>
                    {d}
                  </option>
                ))}
              </select>
            )}
          </div>
        )}

        {kind === "winrateTarget" && (
          <div className="flex gap-2">
            <label className="flex flex-1 flex-col gap-1 text-xs text-slate-400">
              Winrate cible (%)
              <input
                type="number"
                value={percent}
                onChange={(e) => setPercent(Number(e.target.value))}
                className="rounded-xl border border-[var(--color-border-subtle)] bg-[var(--color-surface-2)] px-3 py-2 text-sm text-slate-100"
              />
            </label>
            <label className="flex flex-1 flex-col gap-1 text-xs text-slate-400">
              Parties minimum
              <input
                type="number"
                value={minGames}
                onChange={(e) => setMinGames(Number(e.target.value))}
                className="rounded-xl border border-[var(--color-border-subtle)] bg-[var(--color-surface-2)] px-3 py-2 text-sm text-slate-100"
              />
            </label>
          </div>
        )}

        {kind === "gamesPlayed" && (
          <label className="flex flex-col gap-1 text-xs text-slate-400">
            Nombre de parties à jouer
            <input
              type="number"
              value={count}
              onChange={(e) => setCount(Number(e.target.value))}
              className="rounded-xl border border-[var(--color-border-subtle)] bg-[var(--color-surface-2)] px-3 py-2 text-sm text-slate-100"
            />
          </label>
        )}

        <Button
          size="sm"
          disabled={createObjective.isPending}
          onClick={submit}
          className="self-start"
        >
          {createObjective.isPending ? "Création…" : "Créer l'objectif"}
        </Button>
      </div>
    </Card>
  );
}

export function ObjectivesPage() {
  const activeAccount = useActiveAccountStore((state) => state.activeAccount);
  const { data: objectives, isLoading } = useObjectives(
    activeAccount?.puuid,
    activeAccount?.platform,
  );
  const deleteObjective = useDeleteObjective(activeAccount?.puuid);

  if (!activeAccount) {
    return (
      <div className="mx-auto max-w-lg py-16 text-center">
        <p className="text-slate-400">Liez un compte pour définir des objectifs.</p>
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
    <div className="mx-auto flex max-w-2xl flex-col gap-6 py-8">
      <h1 className="text-2xl font-semibold text-slate-100">Objectifs personnels</h1>

      <NewObjectiveForm puuid={activeAccount.puuid} />

      <Card heading="Vos objectifs" glass className="w-full">
        {isLoading && <p className="text-sm text-slate-400">Chargement…</p>}
        {!isLoading && (!objectives || objectives.length === 0) && (
          <p className="text-sm text-slate-400">Aucun objectif défini pour l'instant.</p>
        )}
        <div className="flex flex-col gap-3">
          {objectives?.map((objective) => (
            <div
              key={objective.id}
              className="rounded-xl border border-[var(--color-border-subtle)] p-3"
            >
              <div className="mb-2 flex items-center justify-between">
                <span className="text-sm font-medium text-slate-100">
                  {describeObjective(objective.kind)}
                </span>
                <div className="flex items-center gap-2">
                  {objective.achievedAt && <Badge tone="win">Atteint</Badge>}
                  <Button
                    size="sm"
                    variant="ghost"
                    onClick={() => deleteObjective.mutate(objective.id)}
                  >
                    Retirer
                  </Button>
                </div>
              </div>
              <div className="h-2 overflow-hidden rounded-full bg-[var(--color-surface-2)]">
                <div
                  className="h-full rounded-full bg-[var(--color-accent-500)] transition-all"
                  style={{ width: `${Math.min(100, objective.progress.percent)}%` }}
                />
              </div>
              <p className="mt-1 text-xs text-slate-500">{objective.progress.summary}</p>
            </div>
          ))}
        </div>
      </Card>
    </div>
  );
}
