import { useState } from "react";

import { ChampionComparison } from "./ChampionComparison";
import { PlayerComparison } from "./PlayerComparison";

const TABS = [
  { key: "players", label: "Joueurs" },
  { key: "champions", label: "Champions" },
] as const;

type TabKey = (typeof TABS)[number]["key"];

export function ComparisonPage() {
  const [tab, setTab] = useState<TabKey>("players");

  return (
    <div className="mx-auto flex max-w-4xl flex-col gap-4 py-8">
      <div>
        <h1 className="text-2xl font-semibold text-slate-100">Comparaison</h1>
        <p className="text-sm text-slate-400">
          Comparez deux joueurs ou deux champions côte à côte, à partir des données Riot officielles
          et des statistiques collectées localement.
        </p>
      </div>

      <div className="flex gap-1.5 border-b border-[var(--color-border-subtle)] pb-2">
        {TABS.map((t) => (
          <button
            key={t.key}
            type="button"
            onClick={() => setTab(t.key)}
            className={`rounded-xl px-3 py-1.5 text-sm transition-colors ${
              tab === t.key
                ? "bg-[var(--color-accent-500)] text-white"
                : "text-slate-400 hover:text-slate-100"
            }`}
          >
            {t.label}
          </button>
        ))}
      </div>

      {tab === "players" ? <PlayerComparison /> : <ChampionComparison />}
    </div>
  );
}
