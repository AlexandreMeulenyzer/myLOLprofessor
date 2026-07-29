import { Link } from "react-router-dom";

import { ROLE_LABELS } from "@/features/champion-select/types";
import { useMatchHistory } from "@/features/history/hooks";
import { Badge } from "@/shared/components/ui/Badge";
import { Card } from "@/shared/components/ui/Card";
import { RemoteIcon } from "@/shared/components/ui/RemoteIcon";
import { useChampionsByKey, useLatestPatchVersion } from "@/features/static-data/hooks";
import { championIconUrl, profileIconUrl, rankEmblemUrl } from "@/shared/lib/data-dragon-assets";
import { useActiveAccountStore } from "@/shared/stores/active-account-store";

import { useProfile } from "./hooks";
import { QUEUE_LABELS } from "./types";

const ROLE_ORDER = ["TOP", "JUNGLE", "MIDDLE", "BOTTOM", "UTILITY"];

export function ProfilePage() {
  const activeAccount = useActiveAccountStore((state) => state.activeAccount);
  const {
    data: profile,
    isLoading,
    isError,
    error,
  } = useProfile(activeAccount?.puuid, activeAccount?.platform);
  const { data: recentMatches } = useMatchHistory(activeAccount?.puuid, 30);
  const championsByKey = useChampionsByKey();
  const { data: version } = useLatestPatchVersion();

  const roleCounts = new Map<string, number>();
  (recentMatches ?? []).forEach((match) => {
    if (match.teamPosition) {
      roleCounts.set(match.teamPosition, (roleCounts.get(match.teamPosition) ?? 0) + 1);
    }
  });
  const totalRoleGames = [...roleCounts.values()].reduce((sum, count) => sum + count, 0);

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
      <div className="flex items-center gap-3">
        {profile && version && (
          <RemoteIcon
            src={profileIconUrl(version, profile.profileIconId)}
            alt="Icône d'invocateur"
            className="h-14 w-14 rounded-full border border-[var(--color-border-subtle)]"
          />
        )}
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
                    className="flex gap-3 rounded-xl border border-[var(--color-border-subtle)] p-3"
                  >
                    <RemoteIcon
                      src={rankEmblemUrl(entry.tier)}
                      alt={entry.tier}
                      className="h-12 w-12 shrink-0"
                    />
                    <div className="min-w-0 flex-1">
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
                      {(entry.hotStreak || entry.veteran || entry.freshBlood) && (
                        <div className="mt-2 flex flex-wrap gap-1.5">
                          {entry.hotStreak && <Badge tone="win">🔥 Hot Streak</Badge>}
                          {entry.freshBlood && <Badge tone="accent">Fresh Blood</Badge>}
                          {entry.veteran && <Badge tone="neutral">Vétéran</Badge>}
                        </div>
                      )}
                    </div>
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
                      {version && champion && (
                        <RemoteIcon
                          src={championIconUrl(version, champion.id)}
                          alt={champion.name}
                          className="h-10 w-10 rounded-full"
                        />
                      )}
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

          {totalRoleGames > 0 && (
            <Card heading="Rôle préféré" glass>
              <p className="mb-2 text-xs text-slate-500">
                Sur les {totalRoleGames} dernières parties synchronisées
              </p>
              <div className="flex flex-col gap-1.5">
                {ROLE_ORDER.filter((role) => roleCounts.has(role)).map((role) => {
                  const count = roleCounts.get(role) ?? 0;
                  const percent = Math.round((count / totalRoleGames) * 100);
                  return (
                    <div key={role} className="flex items-center gap-2 text-sm">
                      <span className="w-16 shrink-0 text-slate-400">
                        {ROLE_LABELS[role] ?? role}
                      </span>
                      <div className="h-2 flex-1 overflow-hidden rounded-full bg-white/10">
                        <div
                          className="h-full rounded-full bg-[var(--color-accent-500)]"
                          style={{ width: `${percent}%` }}
                        />
                      </div>
                      <span className="w-10 shrink-0 text-right text-xs text-slate-500">
                        {percent}%
                      </span>
                    </div>
                  );
                })}
              </div>
            </Card>
          )}
        </>
      )}
    </div>
  );
}
