import { Link } from "react-router-dom";

import { Badge } from "@/shared/components/ui/Badge";
import { Card } from "@/shared/components/ui/Card";
import { useChampionsByKey } from "@/features/static-data/hooks";
import { useActiveAccountStore } from "@/shared/stores/active-account-store";

import { useProfile } from "./hooks";
import { QUEUE_LABELS } from "./types";

export function ProfilePage() {
  const activeAccount = useActiveAccountStore((state) => state.activeAccount);
  const {
    data: profile,
    isLoading,
    isError,
    error,
  } = useProfile(activeAccount?.puuid, activeAccount?.platform);
  const championsByKey = useChampionsByKey();

  if (!activeAccount) {
    return (
      <div className="mx-auto max-w-lg py-16 text-center">
        <p className="text-slate-400">
          Aucun compte actif. Liez un compte Riot pour afficher votre profil.
        </p>
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
    <div className="mx-auto flex max-w-3xl flex-col gap-6 py-8">
      <div>
        <h1 className="text-2xl font-semibold text-slate-100">
          {activeAccount.gameName}
          <span className="text-slate-500">#{activeAccount.tagLine}</span>
        </h1>
        <p className="mt-1 text-sm text-slate-400">
          {activeAccount.platform.toUpperCase()}
          {profile && ` · Niveau ${profile.summonerLevel}`}
        </p>
      </div>

      {isLoading && <p className="text-sm text-slate-400">Chargement du profil…</p>}
      {isError && (
        <p className="text-sm text-[var(--color-loss)]">
          {error instanceof Error ? error.message : String(error)}
        </p>
      )}

      {profile && (
        <>
          <Card heading="Rangs" glass>
            {profile.leagueEntries.length === 0 ? (
              <p className="text-sm text-slate-400">Aucune partie classée cette saison.</p>
            ) : (
              <div className="grid gap-3 sm:grid-cols-2">
                {profile.leagueEntries.map((entry) => (
                  <div
                    key={entry.queueType}
                    className="rounded-xl border border-[var(--color-border-subtle)] p-3"
                  >
                    <div className="mb-1 text-xs font-medium uppercase tracking-wide text-slate-500">
                      {QUEUE_LABELS[entry.queueType] ?? entry.queueType}
                    </div>
                    <div className="text-lg font-semibold text-slate-100">
                      {entry.tier} {entry.rank}
                      <span className="ml-2 text-sm font-normal text-slate-400">
                        {entry.leaguePoints} LP
                      </span>
                    </div>
                    <div className="mt-1 flex items-center gap-2 text-sm text-slate-400">
                      <span>
                        {entry.wins}V / {entry.losses}D
                      </span>
                      <Badge tone={entry.winratePercent >= 50 ? "win" : "loss"}>
                        {entry.winratePercent}% WR
                      </Badge>
                    </div>
                    {entry.estimatedMmr !== null && (
                      <div className="mt-1 text-xs text-slate-500">
                        MMR estimé : {entry.estimatedMmr}
                      </div>
                    )}
                  </div>
                ))}
              </div>
            )}
          </Card>

          <Card heading="Champions principaux" glass>
            {profile.topChampions.length === 0 ? (
              <p className="text-sm text-slate-400">Aucune donnée de maîtrise disponible.</p>
            ) : (
              <div className="flex flex-wrap gap-3">
                {profile.topChampions.map((mastery) => {
                  const champion = championsByKey.get(String(mastery.championId));
                  return (
                    <div
                      key={mastery.championId}
                      className="flex flex-col items-center gap-1 rounded-xl border border-[var(--color-border-subtle)] px-3 py-2"
                    >
                      <span className="text-sm font-medium text-slate-100">
                        {champion?.name ?? `#${mastery.championId}`}
                      </span>
                      <span className="text-xs text-slate-500">
                        Niveau {mastery.championLevel} · {mastery.championPoints.toLocaleString()}{" "}
                        pts
                      </span>
                    </div>
                  );
                })}
              </div>
            )}
          </Card>
        </>
      )}
    </div>
  );
}
