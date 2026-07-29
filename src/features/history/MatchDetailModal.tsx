import {
  useChampionsById,
  useItems,
  useLatestPatchVersion,
  useSummonerSpellsByKey,
} from "@/features/static-data/hooks";
import { Badge } from "@/shared/components/ui/Badge";
import { Modal } from "@/shared/components/ui/Modal";
import { RemoteIcon } from "@/shared/components/ui/RemoteIcon";
import {
  championIconUrl,
  itemIconUrl,
  summonerSpellIconUrl,
} from "@/shared/lib/data-dragon-assets";

import { useMatchDetail } from "./hooks";
import { queueName, type MatchDetailParticipant } from "./types";

function DamageBar({ value, max }: { value: number; max: number }) {
  const width = max > 0 ? Math.max(4, (value / max) * 100) : 0;
  return (
    <div className="h-1.5 w-20 overflow-hidden rounded-full bg-white/10">
      <div
        className="h-full rounded-full bg-[var(--color-accent-500)]"
        style={{ width: `${width}%` }}
      />
    </div>
  );
}

function ParticipantRow({
  participant,
  version,
  durationSeconds,
  maxDamage,
}: {
  participant: MatchDetailParticipant;
  version: string | undefined;
  durationSeconds: number;
  maxDamage: number;
}) {
  const championsById = useChampionsById();
  const summonerSpells = useSummonerSpellsByKey();
  const { data: items } = useItems();
  const champion = championsById.get(participant.championName);
  const cs = participant.totalMinionsKilled + participant.neutralMinionsKilled;
  const csPerMin = durationSeconds > 0 ? (cs / (durationSeconds / 60)).toFixed(1) : "0.0";
  const kda =
    participant.deaths === 0
      ? "Parfait"
      : ((participant.kills + participant.assists) / participant.deaths).toFixed(2);

  return (
    <div className="flex items-center gap-2 border-b border-white/5 px-2 py-1.5 text-xs last:border-b-0">
      <div className="flex w-40 min-w-0 items-center gap-2">
        {version && (
          <RemoteIcon
            src={championIconUrl(version, participant.championName)}
            alt={champion?.name ?? participant.championName}
            className="h-8 w-8 shrink-0 rounded-full"
          />
        )}
        <div className="flex flex-col gap-0.5">
          {version && (
            <div className="flex gap-0.5">
              <RemoteIcon
                src={summonerSpellIconUrl(
                  version,
                  summonerSpells.get(String(participant.summoner1Id))?.id ?? "",
                )}
                alt="Sort 1"
                className="h-3.5 w-3.5 rounded"
              />
              <RemoteIcon
                src={summonerSpellIconUrl(
                  version,
                  summonerSpells.get(String(participant.summoner2Id))?.id ?? "",
                )}
                alt="Sort 2"
                className="h-3.5 w-3.5 rounded"
              />
            </div>
          )}
        </div>
        <div className="min-w-0 truncate">
          <div className="truncate font-medium text-slate-100">
            {champion?.name ?? participant.championName}
          </div>
          {participant.gameName && (
            <div className="truncate text-[10px] text-slate-500">
              {participant.gameName}
              {participant.tagLine && `#${participant.tagLine}`}
            </div>
          )}
        </div>
      </div>

      <div className="w-20 shrink-0 text-center font-mono text-slate-200">
        {participant.kills}/{participant.deaths}/{participant.assists}
        <div className="text-[10px] text-slate-500">{kda}:1</div>
      </div>

      <div className="w-24 shrink-0">
        <div className="font-mono text-slate-300">{participant.totalDamageDealtToChampions}</div>
        <DamageBar value={participant.totalDamageDealtToChampions} max={maxDamage} />
      </div>

      <div className="w-20 shrink-0 text-center font-mono text-slate-400">
        {cs} ({csPerMin})
      </div>

      <div className="w-16 shrink-0 text-center font-mono text-slate-500">
        {participant.wardsPlaced}/{participant.wardsKilled}
      </div>

      <div className="flex flex-1 gap-0.5">
        {participant.items.map((itemId, index) => (
          <div
            key={index}
            className="h-6 w-6 shrink-0 rounded border border-[var(--color-border-subtle)] bg-black/20"
          >
            {itemId > 0 && version && (
              <RemoteIcon
                src={itemIconUrl(version, itemId)}
                alt={items?.[String(itemId)]?.name ?? `Objet #${itemId}`}
                className="h-full w-full rounded"
              />
            )}
          </div>
        ))}
      </div>
    </div>
  );
}

function TeamTable({
  participants,
  version,
  durationSeconds,
  maxDamage,
}: {
  participants: MatchDetailParticipant[];
  version: string | undefined;
  durationSeconds: number;
  maxDamage: number;
}) {
  const won = participants[0]?.win ?? false;

  return (
    <div className="rounded-xl border border-[var(--color-border-subtle)]">
      <div
        className={`px-3 py-1.5 text-xs font-semibold uppercase tracking-wide ${
          won ? "text-[var(--color-win)]" : "text-[var(--color-loss)]"
        }`}
      >
        {won ? "Victoire" : "Défaite"}
      </div>
      {participants.map((participant) => (
        <ParticipantRow
          key={participant.puuid}
          participant={participant}
          version={version}
          durationSeconds={durationSeconds}
          maxDamage={maxDamage}
        />
      ))}
    </div>
  );
}

export function MatchDetailModal({
  matchId,
  platform,
  onClose,
}: {
  matchId: string;
  platform: string;
  onClose: () => void;
}) {
  const { data: match, isLoading, isError, error } = useMatchDetail(matchId, platform);
  const { data: version } = useLatestPatchVersion();

  const teamOrder = match?.participants.filter((p) => p.teamId === 100) ?? [];
  const teamChaos = match?.participants.filter((p) => p.teamId === 200) ?? [];
  const maxDamage = match
    ? Math.max(...match.participants.map((p) => p.totalDamageDealtToChampions), 1)
    : 1;

  return (
    <Modal title={match ? queueName(match.queueId) : "Détail du match"} onClose={onClose}>
      {isLoading && <p className="text-sm text-slate-400">Chargement du détail du match…</p>}
      {isError && (
        <p className="text-sm text-[var(--color-loss)]">
          {error instanceof Error ? error.message : String(error)}
        </p>
      )}
      {match && (
        <div className="flex flex-col gap-3">
          <div className="flex items-center gap-2 text-xs text-slate-500">
            <Badge tone="neutral">{Math.round(match.durationSeconds / 60)} min</Badge>
          </div>
          <TeamTable
            participants={teamOrder}
            version={version}
            durationSeconds={match.durationSeconds}
            maxDamage={maxDamage}
          />
          <TeamTable
            participants={teamChaos}
            version={version}
            durationSeconds={match.durationSeconds}
            maxDamage={maxDamage}
          />
        </div>
      )}
    </Modal>
  );
}
