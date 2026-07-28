import { useState } from "react";

import { PLATFORMS } from "@/features/accounts/types";
import { useProfile } from "@/features/profile/hooks";
import { QUEUE_LABELS } from "@/features/profile/types";
import { useResolveSummoner } from "@/features/search/hooks";
import { useChampionsByKey } from "@/features/static-data/hooks";
import { Card } from "@/shared/components/ui/Card";
import { Button } from "@/shared/components/ui/Button";

function PlayerSlot({ label }: { label: string }) {
  const [gameName, setGameName] = useState("");
  const [tagLine, setTagLine] = useState("");
  const [platform, setPlatform] = useState(PLATFORMS[0]?.value ?? "euw1");
  const resolveSummoner = useResolveSummoner();
  const { data: profile } = useProfile(resolveSummoner.data?.puuid, platform);
  const championsByKey = useChampionsByKey();

  return (
    <Card heading={label} glass className="flex-1">
      <form
        className="mb-3 flex flex-wrap gap-1.5"
        onSubmit={(event) => {
          event.preventDefault();
          if (!gameName.trim() || !tagLine.trim()) return;
          resolveSummoner.mutate({ gameName: gameName.trim(), tagLine: tagLine.trim(), platform });
        }}
      >
        <input
          value={gameName}
          onChange={(e) => setGameName(e.target.value)}
          placeholder="Invocateur"
          className="min-w-0 flex-1 rounded-xl border border-[var(--color-border-subtle)] bg-[var(--color-surface-2)] px-2 py-1.5 text-sm text-slate-100"
        />
        <input
          value={tagLine}
          onChange={(e) => setTagLine(e.target.value)}
          placeholder="TAG"
          className="w-16 rounded-xl border border-[var(--color-border-subtle)] bg-[var(--color-surface-2)] px-2 py-1.5 text-sm text-slate-100"
        />
        <select
          value={platform}
          onChange={(e) => setPlatform(e.target.value)}
          className="rounded-xl border border-[var(--color-border-subtle)] bg-[var(--color-surface-2)] px-2 py-1.5 text-xs text-slate-100"
        >
          {PLATFORMS.map((p) => (
            <option key={p.value} value={p.value}>
              {p.value.toUpperCase()}
            </option>
          ))}
        </select>
        <Button size="sm" type="submit" disabled={resolveSummoner.isPending}>
          OK
        </Button>
      </form>

      {profile && (
        <div className="flex flex-col gap-2 text-sm">
          <div className="font-medium text-slate-100">
            {resolveSummoner.data?.gameName}
            <span className="text-slate-500">#{resolveSummoner.data?.tagLine}</span>
          </div>
          <div className="text-slate-400">Niveau {profile.summonerLevel}</div>
          {profile.leagueEntries.map((entry) => (
            <div key={entry.queueType} className="text-xs text-slate-400">
              {QUEUE_LABELS[entry.queueType] ?? entry.queueType} : {entry.tier} {entry.rank} (
              {entry.leaguePoints} LP) — {entry.winratePercent}% WR
            </div>
          ))}
          {profile.leagueEntries.length === 0 && (
            <div className="text-xs text-slate-500">Non classé</div>
          )}
          <div className="text-xs text-slate-500">
            Top champions :{" "}
            {profile.topChampions
              .map((c) => championsByKey.get(String(c.championId))?.name ?? `#${c.championId}`)
              .join(", ") || "—"}
          </div>
        </div>
      )}
    </Card>
  );
}

export function PlayerComparison() {
  return (
    <div className="flex flex-col gap-4 sm:flex-row">
      <PlayerSlot label="Joueur A" />
      <PlayerSlot label="Joueur B" />
    </div>
  );
}
