import { useMemo } from "react";

import { useChampionsByKey, useLatestPatchVersion } from "@/features/static-data/hooks";
import { Badge } from "@/shared/components/ui/Badge";
import { Card } from "@/shared/components/ui/Card";
import { RemoteIcon } from "@/shared/components/ui/RemoteIcon";
import { championIconUrl } from "@/shared/lib/data-dragon-assets";
import { useActiveAccountStore } from "@/shared/stores/active-account-store";
import { useGamePhaseStore } from "@/shared/stores/game-phase-store";

import { computeCompositionProfile, tankinessLabel } from "./composition";
import { useChampSelectTeamAnalysis } from "./hooks";
import { ROLE_LABELS, type ParticipantAnalysis, type TeamSide } from "./types";

function average(values: number[]): number | null {
  if (values.length === 0) return null;
  return Math.round(values.reduce((sum, v) => sum + v, 0) / values.length);
}

function TagCounts({ tagCounts }: { tagCounts: Map<string, number> }) {
  if (tagCounts.size === 0) return <span className="text-slate-500">—</span>;
  return (
    <div className="flex flex-wrap gap-1.5">
      {[...tagCounts.entries()].map(([tag, count]) => (
        <Badge key={tag} tone="neutral">
          {tag} ×{count}
        </Badge>
      ))}
    </div>
  );
}

// Seuil au-dela duquel la concentration de maitrise sur un seul champion est
// consideree comme un signal "One Trick Pony" fiable (evite les faux
// positifs sur des joueurs avec un pool equilibre).
const OTP_MASTERY_SHARE_THRESHOLD = 55;

function ParticipantRow({
  participant,
  championName,
  championDataDragonId,
  version,
}: {
  participant: ParticipantAnalysis;
  championName: string;
  championDataDragonId: string | undefined;
  version: string | undefined;
}) {
  const isOtp =
    participant.isPlayingTopMasteryChampion &&
    (participant.topChampionMasterySharePercent ?? 0) >= OTP_MASTERY_SHARE_THRESHOLD;

  return (
    <div
      className={`flex items-center justify-between gap-2 rounded-xl border px-3 py-2 ${
        participant.isLocalPlayer
          ? "border-[var(--color-accent-500)] bg-[var(--color-accent-500)]/5"
          : "border-[var(--color-border-subtle)]"
      }`}
    >
      {version && championDataDragonId && (
        <RemoteIcon
          src={championIconUrl(version, championDataDragonId)}
          alt={championName}
          className="h-9 w-9 shrink-0 rounded-full border border-[var(--color-border-subtle)]"
        />
      )}
      <div className="min-w-0 flex-1">
        <div className="text-sm font-medium text-slate-100">
          {participant.gameName || "?"}
          {participant.tagLine && <span className="text-slate-500">#{participant.tagLine}</span>}
        </div>
        <div className="text-xs text-slate-500">
          {championName} · {ROLE_LABELS[participant.role] ?? (participant.role || "—")}
        </div>
        {(isOtp || participant.hotStreak || participant.freshBlood) && (
          <div className="mt-1 flex flex-wrap gap-1">
            {isOtp && (
              <Badge tone="accent">OTP {participant.topChampionMasterySharePercent}%</Badge>
            )}
            {participant.hotStreak && <Badge tone="win">🔥 Hot Streak</Badge>}
            {participant.freshBlood && <Badge tone="neutral">Fresh Blood</Badge>}
          </div>
        )}
      </div>
      <div className="text-right text-xs">
        {participant.tier ? (
          <>
            <div className="text-slate-200">
              {participant.tier} {participant.rank} ({participant.leaguePoints} LP)
            </div>
            <div className="text-slate-500">
              {participant.winratePercent}% WR · MMR est. {participant.estimatedMmr ?? "—"}
            </div>
          </>
        ) : (
          <div className="text-slate-500">Non classé</div>
        )}
      </div>
    </div>
  );
}

function TeamColumn({
  title,
  participants,
  championsByKey,
  version,
}: {
  title: string;
  participants: ParticipantAnalysis[];
  championsByKey: ReturnType<typeof useChampionsByKey>;
  version: string | undefined;
}) {
  const mmrValues = participants.map((p) => p.estimatedMmr).filter((v): v is number => v !== null);
  const powerScore = average(mmrValues);

  const tagCounts = useMemo(() => {
    const counts = new Map<string, number>();
    participants.forEach((p) => {
      const champion = championsByKey.get(String(p.championId));
      champion?.tags.forEach((tag) => counts.set(tag, (counts.get(tag) ?? 0) + 1));
    });
    return counts;
  }, [participants, championsByKey]);

  const compositionProfile = useMemo(() => {
    const infos = participants
      .map((p) => championsByKey.get(String(p.championId))?.info)
      .filter((info): info is NonNullable<typeof info> => info !== undefined);
    return computeCompositionProfile(infos);
  }, [participants, championsByKey]);

  return (
    <div className="flex flex-col gap-2">
      <div className="flex items-center justify-between">
        <h2 className="text-sm font-semibold uppercase tracking-wide text-slate-400">{title}</h2>
        {powerScore !== null && (
          <span className="text-xs text-slate-500">Force estimée : {powerScore}</span>
        )}
      </div>
      <Card glass>
        <div className="mb-3">
          <div className="mb-1 text-xs uppercase tracking-wide text-slate-500">Composition</div>
          <TagCounts tagCounts={tagCounts} />
          {compositionProfile && (
            <div className="mt-2 flex flex-wrap items-center gap-1.5 text-xs text-slate-400">
              <Badge tone="neutral">{compositionProfile.adCount} AD</Badge>
              <Badge tone="neutral">{compositionProfile.apCount} AP</Badge>
              {compositionProfile.mixedCount > 0 && (
                <Badge tone="neutral">{compositionProfile.mixedCount} Mixte</Badge>
              )}
              <span>
                Tankiness : {tankinessLabel(compositionProfile.averageTankiness)} (
                {compositionProfile.averageTankiness}/10)
              </span>
            </div>
          )}
        </div>
        <div className="flex flex-col gap-1.5">
          {participants.map((participant) => {
            const champion = championsByKey.get(String(participant.championId));
            return (
              <ParticipantRow
                key={participant.puuid}
                participant={participant}
                championName={champion?.name ?? `#${participant.championId}`}
                championDataDragonId={champion?.id}
                version={version}
              />
            );
          })}
        </div>
      </Card>
    </div>
  );
}

export function TeamAnalysisPage() {
  const phase = useGamePhaseStore((state) => state.phase);
  const activeAccount = useActiveAccountStore((state) => state.activeAccount);
  const {
    data: analyses,
    isLoading,
    isError,
    error,
  } = useChampSelectTeamAnalysis(activeAccount?.platform);
  const championsByKey = useChampionsByKey();
  const { data: version } = useLatestPatchVersion();

  if (phase !== "ChampSelect") {
    return (
      <div className="mx-auto max-w-lg py-16 text-center text-slate-400">
        <p>L'analyse d'équipe s'affiche automatiquement pendant la sélection des champions.</p>
      </div>
    );
  }

  if (isLoading) {
    return (
      <div className="mx-auto max-w-lg py-16 text-center text-slate-400">
        <p>Analyse des 10 joueurs en cours…</p>
      </div>
    );
  }

  if (isError) {
    return (
      <div className="mx-auto max-w-lg py-16 text-center text-[var(--color-loss)]">
        <p>{error instanceof Error ? error.message : String(error)}</p>
      </div>
    );
  }

  const groupBySide = (side: TeamSide) => (analyses ?? []).filter((p) => p.side === side);
  const allies = groupBySide("ally");
  const enemies = groupBySide("enemy");

  return (
    <div className="mx-auto flex max-w-4xl flex-col gap-6 py-8">
      <h1 className="text-2xl font-semibold text-slate-100">Analyse des équipes</h1>

      {allies.length === 0 && enemies.length === 0 ? (
        <p className="text-sm text-slate-400">
          Aucun joueur identifiable pour l'instant — les identités adverses ne sont pas toujours
          visibles selon le mode de jeu.
        </p>
      ) : (
        <div className="grid gap-6 md:grid-cols-2">
          <TeamColumn
            title="Votre équipe"
            participants={allies}
            championsByKey={championsByKey}
            version={version}
          />
          <TeamColumn
            title="Équipe adverse"
            participants={enemies}
            championsByKey={championsByKey}
            version={version}
          />
        </div>
      )}
    </div>
  );
}
