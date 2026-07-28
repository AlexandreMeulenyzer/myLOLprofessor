import { Link } from "react-router-dom";

import { useHasRiotApiKey } from "@/features/accounts/hooks";
import { useMatchHistory } from "@/features/history/hooks";
import { queueName } from "@/features/history/types";
import { useProfile } from "@/features/profile/hooks";
import { QUEUE_LABELS } from "@/features/profile/types";
import { useLatestPatchVersion } from "@/features/static-data/hooks";
import { Badge } from "@/shared/components/ui/Badge";
import { Button } from "@/shared/components/ui/Button";
import { Card } from "@/shared/components/ui/Card";
import { useActiveAccountStore } from "@/shared/stores/active-account-store";

export function DashboardPage() {
  const { data: hasApiKey, isLoading: loadingKey } = useHasRiotApiKey();
  const activeAccount = useActiveAccountStore((state) => state.activeAccount);
  const { data: profile } = useProfile(activeAccount?.puuid, activeAccount?.platform);
  const { data: matches } = useMatchHistory(activeAccount?.puuid, 10);
  const { data: patchVersion } = useLatestPatchVersion();

  if (!loadingKey && !hasApiKey) {
    return (
      <div className="mx-auto max-w-lg py-16 text-center">
        <h1 className="mb-2 text-xl font-semibold text-slate-100">Bienvenue sur Wardstone</h1>
        <p className="mb-4 text-sm text-slate-400">
          Configurez votre clé API Riot personnelle pour commencer à suivre vos statistiques.
        </p>
        <Link to="/accounts">
          <Button>Configurer maintenant</Button>
        </Link>
      </div>
    );
  }

  if (!activeAccount) {
    return (
      <div className="mx-auto max-w-lg py-16 text-center">
        <h1 className="mb-2 text-xl font-semibold text-slate-100">Liez votre premier compte</h1>
        <p className="mb-4 text-sm text-slate-400">
          Ajoutez votre Riot ID pour afficher votre dashboard.
        </p>
        <Link to="/accounts">
          <Button>Lier un compte</Button>
        </Link>
      </div>
    );
  }

  const soloQueue = profile?.leagueEntries.find((e) => e.queueType === "RANKED_SOLO_5x5");
  const recentForm = matches?.slice(0, 10) ?? [];
  const winrateOnForm = recentForm.length
    ? Math.round((recentForm.filter((m) => m.win).length / recentForm.length) * 100)
    : null;

  const championCounts = new Map<string, number>();
  recentForm.forEach((m) =>
    championCounts.set(m.champion, (championCounts.get(m.champion) ?? 0) + 1),
  );
  const topChampion = [...championCounts.entries()].sort((a, b) => b[1] - a[1])[0]?.[0];

  return (
    <div className="mx-auto flex max-w-4xl flex-col gap-6 py-8">
      <div className="flex items-center justify-between">
        <div>
          <h1 className="text-2xl font-semibold text-slate-100">
            {activeAccount.gameName}
            <span className="text-slate-500">#{activeAccount.tagLine}</span>
          </h1>
          <p className="text-sm text-slate-400">Patch {patchVersion ?? "…"}</p>
        </div>
        <Link to="/profile" className="text-sm text-[var(--color-accent-400)] hover:underline">
          Voir le profil complet →
        </Link>
      </div>

      <div className="grid gap-4 sm:grid-cols-3">
        <Card heading="Rang Solo/Duo" glass>
          {soloQueue ? (
            <>
              <div className="text-lg font-semibold text-slate-100">
                {soloQueue.tier} {soloQueue.rank}
              </div>
              <div className="text-sm text-slate-400">{soloQueue.leaguePoints} LP</div>
            </>
          ) : (
            <p className="text-sm text-slate-400">Non classé</p>
          )}
        </Card>

        <Card heading="Forme récente" glass>
          {winrateOnForm !== null ? (
            <>
              <div className="text-lg font-semibold text-slate-100">{winrateOnForm}% WR</div>
              <div className="text-sm text-slate-400">
                Sur les {recentForm.length} dernières parties synchronisées
              </div>
            </>
          ) : (
            <p className="text-sm text-slate-400">
              Aucune partie synchronisée — voir la page Historique.
            </p>
          )}
        </Card>

        <Card heading="Champion du moment" glass>
          {topChampion ? (
            <div className="text-lg font-semibold text-slate-100">{topChampion}</div>
          ) : (
            <p className="text-sm text-slate-400">—</p>
          )}
        </Card>
      </div>

      <Card heading="Dernières parties" glass>
        {recentForm.length === 0 ? (
          <p className="text-sm text-slate-400">
            Rendez-vous sur{" "}
            <Link to="/history" className="text-[var(--color-accent-400)] hover:underline">
              Historique
            </Link>{" "}
            pour synchroniser vos parties récentes.
          </p>
        ) : (
          <ul className="flex flex-col gap-1.5">
            {recentForm.slice(0, 5).map((match) => (
              <li
                key={match.matchId}
                className="flex items-center justify-between text-sm text-slate-300"
              >
                <span className="flex items-center gap-2">
                  <Badge tone={match.win ? "win" : "loss"}>{match.win ? "V" : "D"}</Badge>
                  {match.champion}
                </span>
                <span className="text-slate-500">{queueName(match.queueId)}</span>
              </li>
            ))}
          </ul>
        )}
      </Card>

      {soloQueue === undefined && profile && profile.leagueEntries.length > 0 && (
        <p className="text-xs text-slate-500">
          File(s) classée(s) disponible(s) :{" "}
          {profile.leagueEntries.map((e) => QUEUE_LABELS[e.queueType] ?? e.queueType).join(", ")}
        </p>
      )}
    </div>
  );
}
