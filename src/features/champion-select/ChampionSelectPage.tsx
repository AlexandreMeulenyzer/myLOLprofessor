import { Badge } from "@/shared/components/ui/Badge";
import { Card } from "@/shared/components/ui/Card";
import { RemoteIcon } from "@/shared/components/ui/RemoteIcon";
import {
  useChampionDetail,
  useChampionsByKey,
  useItems,
  useLatestPatchVersion,
  useRunesById,
  useSummonerSpellsByKey,
} from "@/features/static-data/hooks";
import { championIconUrl } from "@/shared/lib/data-dragon-assets";
import { useGamePhaseStore } from "@/shared/stores/game-phase-store";

import { useChampionRoleStats, useChampSelectSelection } from "./hooks";
import { ROLE_LABELS } from "./types";

function StatTile({ label, value }: { label: string; value: string }) {
  return (
    <div className="rounded-xl border border-[var(--color-border-subtle)] px-3 py-2 text-center">
      <div className="text-lg font-semibold text-slate-100">{value}</div>
      <div className="text-xs text-slate-500">{label}</div>
    </div>
  );
}

export function ChampionSelectPage() {
  const phase = useGamePhaseStore((state) => state.phase);
  const { data: selection } = useChampSelectSelection();
  const championsByKey = useChampionsByKey();
  const { data: patchVersion } = useLatestPatchVersion();
  const { data: items } = useItems();
  const summonerSpells = useSummonerSpellsByKey();
  const runesById = useRunesById();

  const championSummary = selection ? championsByKey.get(String(selection.championId)) : undefined;
  const { data: championDetail } = useChampionDetail(championSummary?.id);
  const { data: stats } = useChampionRoleStats(
    selection?.championId,
    selection?.role,
    patchVersion,
  );

  if (phase !== "ChampSelect") {
    return (
      <div className="mx-auto max-w-lg py-16 text-center text-slate-400">
        <p>L'assistant s'affichera automatiquement dès votre entrée en sélection de champion.</p>
      </div>
    );
  }

  if (!selection) {
    return (
      <div className="mx-auto max-w-lg py-16 text-center text-slate-400">
        <p>En attente du choix d'un champion…</p>
      </div>
    );
  }

  return (
    <div className="mx-auto flex max-w-3xl flex-col gap-5 py-8">
      <div className="flex items-center gap-3">
        {patchVersion && championSummary && (
          <RemoteIcon
            src={championIconUrl(patchVersion, championSummary.id)}
            alt={championSummary.name}
            className="h-14 w-14 rounded-full border border-[var(--color-border-subtle)]"
          />
        )}
        <div>
          <h1 className="text-2xl font-semibold text-slate-100">
            {championSummary?.name ?? `Champion #${selection.championId}`}
          </h1>
          {championDetail && <p className="text-sm text-slate-400">{championDetail.title}</p>}
        </div>
        {selection.role && (
          <Badge tone="accent">{ROLE_LABELS[selection.role] ?? selection.role}</Badge>
        )}
      </div>

      {stats ? (
        <Card heading="Statistiques (données collectées localement)" glass>
          <div className="grid grid-cols-2 gap-2 sm:grid-cols-4">
            <StatTile label="Winrate" value={`${stats.winratePercent}%`} />
            <StatTile
              label="Pickrate"
              value={stats.pickratePercent !== null ? `${stats.pickratePercent}%` : "—"}
            />
            <StatTile
              label="Banrate"
              value={stats.banratePercent !== null ? `${stats.banratePercent}%` : "—"}
            />
            <StatTile
              label="Durée moyenne"
              value={`${Math.round(stats.avgGameDurationSeconds / 60)} min`}
            />
          </div>
          <div className="mt-2 text-center text-xs text-slate-500">
            KDA moyen : {stats.avgKills} / {stats.avgDeaths} / {stats.avgAssists} — CS/min{" "}
            {stats.avgCsPerMin} — Or/min {stats.avgGoldPerMin} — sur {stats.games} partie(s)
            synchronisée(s)
          </div>

          {stats.commonKeystones.length > 0 && (
            <div className="mt-4">
              <div className="mb-1 text-xs font-medium uppercase tracking-wide text-slate-500">
                Rune principale la plus jouée
              </div>
              <div className="flex flex-wrap gap-2">
                {stats.commonKeystones.map(([runeId, count]) => (
                  <Badge key={runeId} tone="neutral">
                    {runesById.get(runeId)?.name ?? `Rune #${runeId}`} ({count})
                  </Badge>
                ))}
              </div>
            </div>
          )}

          {stats.commonSummonerSpells.length > 0 && (
            <div className="mt-3">
              <div className="mb-1 text-xs font-medium uppercase tracking-wide text-slate-500">
                Sorts d'invocateur
              </div>
              <div className="flex flex-wrap gap-2">
                {stats.commonSummonerSpells.map(([spellId, count]) => (
                  <Badge key={spellId} tone="neutral">
                    {summonerSpells.get(String(spellId))?.name ?? `Sort #${spellId}`} ({count})
                  </Badge>
                ))}
              </div>
            </div>
          )}

          {stats.commonItems.length > 0 && (
            <div className="mt-3">
              <div className="mb-1 text-xs font-medium uppercase tracking-wide text-slate-500">
                Objets fréquents
              </div>
              <div className="flex flex-wrap gap-2">
                {stats.commonItems.map(([itemId, count]) => (
                  <Badge key={itemId} tone="neutral">
                    {items?.[String(itemId)]?.name ?? `Objet #${itemId}`} ({count})
                  </Badge>
                ))}
              </div>
            </div>
          )}
        </Card>
      ) : (
        <Card heading="Statistiques" glass>
          <p className="text-sm text-slate-400">
            Pas encore assez de données collectées pour ce champion/rôle/patch. Synchronisez votre
            historique (page Historique) ou jouez davantage de parties : la couverture s'améliore
            avec l'usage du logiciel.
          </p>
        </Card>
      )}

      {championDetail && (
        <Card heading="Conseils" glass>
          <div className="mb-2 flex flex-wrap gap-1.5">
            {championDetail.tags.map((tag) => (
              <Badge key={tag} tone="neutral">
                {tag}
              </Badge>
            ))}
            <Badge tone="warning">Difficulté {championDetail.info.difficulty}/10</Badge>
          </div>
          {championDetail.allytips.length > 0 && (
            <ul className="mb-2 list-inside list-disc text-sm text-slate-300">
              {championDetail.allytips.map((tip, index) => (
                <li key={index}>{tip}</li>
              ))}
            </ul>
          )}
          {championDetail.enemytips.length > 0 && (
            <>
              <div className="mb-1 text-xs font-medium uppercase tracking-wide text-slate-500">
                Contre ce champion
              </div>
              <ul className="list-inside list-disc text-sm text-slate-400">
                {championDetail.enemytips.map((tip, index) => (
                  <li key={index}>{tip}</li>
                ))}
              </ul>
            </>
          )}
        </Card>
      )}
    </div>
  );
}
