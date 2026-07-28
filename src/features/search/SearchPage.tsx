import { useMemo, useState } from "react";
import { Link } from "react-router-dom";

import { PLATFORMS } from "@/features/accounts/types";
import {
  useChampions,
  useItems,
  useLatestPatchVersion,
  useRunes,
  useSummonerSpells,
} from "@/features/static-data/hooks";
import { Badge } from "@/shared/components/ui/Badge";
import { Button } from "@/shared/components/ui/Button";
import { Card } from "@/shared/components/ui/Card";

import { useResolveSummoner } from "./hooks";

function SummonerSearch() {
  const [gameName, setGameName] = useState("");
  const [tagLine, setTagLine] = useState("");
  const [platform, setPlatform] = useState(PLATFORMS[0]?.value ?? "euw1");
  const resolveSummoner = useResolveSummoner();

  return (
    <Card heading="Invocateur" glass>
      <form
        className="flex flex-wrap items-center gap-2"
        onSubmit={(event) => {
          event.preventDefault();
          if (!gameName.trim() || !tagLine.trim()) return;
          resolveSummoner.mutate({ gameName: gameName.trim(), tagLine: tagLine.trim(), platform });
        }}
      >
        <input
          value={gameName}
          onChange={(e) => setGameName(e.target.value)}
          placeholder="Nom d'invocateur"
          className="rounded-xl border border-[var(--color-border-subtle)] bg-[var(--color-surface-2)] px-3 py-2 text-sm text-slate-100"
        />
        <span className="text-slate-500">#</span>
        <input
          value={tagLine}
          onChange={(e) => setTagLine(e.target.value)}
          placeholder="TAG"
          className="w-20 rounded-xl border border-[var(--color-border-subtle)] bg-[var(--color-surface-2)] px-3 py-2 text-sm text-slate-100"
        />
        <select
          value={platform}
          onChange={(e) => setPlatform(e.target.value)}
          className="rounded-xl border border-[var(--color-border-subtle)] bg-[var(--color-surface-2)] px-3 py-2 text-sm text-slate-100"
        >
          {PLATFORMS.map((p) => (
            <option key={p.value} value={p.value}>
              {p.label}
            </option>
          ))}
        </select>
        <Button size="sm" type="submit" disabled={resolveSummoner.isPending}>
          {resolveSummoner.isPending ? "Recherche…" : "Rechercher"}
        </Button>
      </form>

      {resolveSummoner.isError && (
        <p className="mt-2 text-xs text-[var(--color-loss)]">
          {resolveSummoner.error instanceof Error
            ? resolveSummoner.error.message
            : String(resolveSummoner.error)}
        </p>
      )}

      {resolveSummoner.data && (
        <div className="mt-3 flex items-center justify-between rounded-xl border border-[var(--color-border-subtle)] px-3 py-2">
          <span className="text-sm text-slate-100">
            {resolveSummoner.data.gameName}
            <span className="text-slate-500">#{resolveSummoner.data.tagLine}</span>
          </span>
          <span className="text-xs text-slate-500">
            Niveau {resolveSummoner.data.summonerLevel} ·{" "}
            {resolveSummoner.data.platform.toUpperCase()}
          </span>
        </div>
      )}
    </Card>
  );
}

export function SearchPage() {
  const [query, setQuery] = useState("");
  const { data: champions } = useChampions();
  const { data: items } = useItems();
  const { data: runeTrees } = useRunes();
  const { data: summonerSpells } = useSummonerSpells();
  const { data: patchVersion } = useLatestPatchVersion();

  const normalized = query.trim().toLowerCase();

  const matchedChampions = useMemo(
    () =>
      normalized.length < 2
        ? []
        : (champions ?? []).filter((c) => c.name.toLowerCase().includes(normalized)).slice(0, 8),
    [champions, normalized],
  );

  const matchedItems = useMemo(() => {
    if (normalized.length < 2 || !items) return [];
    return Object.entries(items)
      .filter(([, item]) => item.name.toLowerCase().includes(normalized))
      .slice(0, 8);
  }, [items, normalized]);

  const matchedRunes = useMemo(() => {
    if (normalized.length < 2 || !runeTrees) return [];
    return runeTrees
      .flatMap((tree) => tree.slots.flatMap((slot) => slot.runes))
      .filter((rune) => rune.name.toLowerCase().includes(normalized))
      .slice(0, 8);
  }, [runeTrees, normalized]);

  const matchedSpells = useMemo(() => {
    if (normalized.length < 2 || !summonerSpells) return [];
    return Object.values(summonerSpells)
      .filter((spell) => spell.name.toLowerCase().includes(normalized))
      .slice(0, 8);
  }, [summonerSpells, normalized]);

  const hasResults =
    matchedChampions.length > 0 ||
    matchedItems.length > 0 ||
    matchedRunes.length > 0 ||
    matchedSpells.length > 0;

  return (
    <div className="mx-auto flex max-w-2xl flex-col gap-4 py-8">
      <h1 className="text-2xl font-semibold text-slate-100">Recherche globale</h1>

      <SummonerSearch />

      <Card heading="Champions, objets, runes, sorts" glass>
        <input
          value={query}
          onChange={(e) => setQuery(e.target.value)}
          placeholder="Rechercher (ex : Ahri, Lame d'Infini, Électrocution...)"
          className="mb-3 w-full rounded-xl border border-[var(--color-border-subtle)] bg-[var(--color-surface-2)] px-3 py-2 text-sm text-slate-100 outline-none focus:border-[var(--color-accent-500)]"
        />

        {normalized.length >= 2 && !hasResults && (
          <p className="text-sm text-slate-400">Aucun résultat pour « {query} ».</p>
        )}

        {matchedChampions.length > 0 && (
          <div className="mb-3">
            <div className="mb-1 text-xs uppercase tracking-wide text-slate-500">Champions</div>
            <div className="flex flex-wrap gap-1.5">
              {matchedChampions.map((champion) => (
                <Link key={champion.id} to="/champion-select">
                  <Badge tone="accent">{champion.name}</Badge>
                </Link>
              ))}
            </div>
          </div>
        )}

        {matchedItems.length > 0 && (
          <div className="mb-3">
            <div className="mb-1 text-xs uppercase tracking-wide text-slate-500">Objets</div>
            <div className="flex flex-wrap gap-1.5">
              {matchedItems.map(([id, item]) => (
                <Badge key={id} tone="neutral">
                  {item.name}
                </Badge>
              ))}
            </div>
          </div>
        )}

        {matchedRunes.length > 0 && (
          <div className="mb-3">
            <div className="mb-1 text-xs uppercase tracking-wide text-slate-500">Runes</div>
            <div className="flex flex-wrap gap-1.5">
              {matchedRunes.map((rune) => (
                <Badge key={rune.id} tone="neutral">
                  {rune.name}
                </Badge>
              ))}
            </div>
          </div>
        )}

        {matchedSpells.length > 0 && (
          <div>
            <div className="mb-1 text-xs uppercase tracking-wide text-slate-500">
              Sorts d'invocateur
            </div>
            <div className="flex flex-wrap gap-1.5">
              {matchedSpells.map((spell) => (
                <Badge key={spell.id} tone="neutral">
                  {spell.name}
                </Badge>
              ))}
            </div>
          </div>
        )}
      </Card>

      <p className="text-xs text-slate-500">Patch en cours : {patchVersion ?? "…"}</p>
    </div>
  );
}
