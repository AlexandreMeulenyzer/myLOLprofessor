import { useLiveGameSnapshot } from "@/features/live-game/hooks";
import { formatGameClock, formatTimer, type LivePlayerSnapshot } from "@/features/live-game/types";
import {
  useChampionsByKey,
  useChampionsById,
  useLatestPatchVersion,
} from "@/features/static-data/hooks";
import { useGamePhaseSync } from "@/shared/hooks/useGamePhaseSync";
import { championIconUrl } from "@/shared/lib/data-dragon-assets";
import { invoke } from "@/shared/lib/tauri-bridge";
import { RemoteIcon } from "@/shared/components/ui/RemoteIcon";
import { useChampSelectRosterStore } from "@/shared/stores/champ-select-roster-store";
import { useGamePhaseStore } from "@/shared/stores/game-phase-store";
import { useOverlaySettingsStore } from "@/shared/stores/overlay-settings-store";
import { GAME_PHASE_LABELS } from "@/shared/types/game-phase";

function closeOverlay() {
  void invoke("toggle_overlay_window");
}

function csPerMinute(creepScore: number, gameTimeSeconds: number): string {
  if (gameTimeSeconds <= 0) return "0.0";
  return (creepScore / (gameTimeSeconds / 60)).toFixed(1);
}

function ScoreboardRow({
  player,
  gameTimeSeconds,
  version,
  reversed,
}: {
  player: LivePlayerSnapshot;
  gameTimeSeconds: number;
  version: string | undefined;
  reversed?: boolean;
}) {
  const championsById = useChampionsById();
  const champion = championsById.get(player.championName);

  return (
    <div
      className={`flex items-center gap-1.5 rounded-md px-1 py-0.5 text-[11px] ${
        player.isDead ? "opacity-40" : ""
      } ${reversed ? "flex-row-reverse text-right" : ""}`}
    >
      {version && (
        <RemoteIcon
          src={championIconUrl(version, player.championName)}
          alt={champion?.name ?? player.championName}
          className="h-5 w-5 shrink-0 rounded-full"
        />
      )}
      <span className="min-w-0 flex-1 truncate text-slate-200">
        {champion?.name ?? player.championName}
      </span>
      <span className="shrink-0 font-mono text-slate-300">
        {player.kills}/{player.deaths}/{player.assists}
      </span>
      <span className="shrink-0 font-mono text-slate-500">
        {player.creepScore} ({csPerMinute(player.creepScore, gameTimeSeconds)})
      </span>
    </div>
  );
}

function DeltaStat({
  label,
  mine,
  theirs,
  format,
}: {
  label: string;
  mine: number;
  theirs: number;
  format: (value: number) => string;
}) {
  const diff = mine - theirs;
  const tone =
    diff > 0 ? "text-[var(--color-win)]" : diff < 0 ? "text-[var(--color-loss)]" : "text-slate-400";

  return (
    <div className="flex flex-col items-center">
      <span className="text-[9px] uppercase tracking-wide text-slate-500">{label}</span>
      <span className="font-mono text-xs text-slate-200">{format(mine)}</span>
      <span className={`font-mono text-[10px] ${tone}`}>
        {diff > 0 ? "+" : ""}
        {format(diff)}
      </span>
    </div>
  );
}

function LaneMatchup({
  me,
  opponent,
  gameTimeSeconds,
  version,
}: {
  me: LivePlayerSnapshot;
  opponent: LivePlayerSnapshot;
  gameTimeSeconds: number;
  version: string | undefined;
}) {
  const championsById = useChampionsById();

  return (
    <div className="border-t border-white/10 pt-2">
      <div className="mb-1 text-center text-[10px] uppercase tracking-wide text-slate-500">
        Face-à-face de lane
      </div>
      <div className="flex items-center justify-between gap-2">
        {version && (
          <RemoteIcon
            src={championIconUrl(version, me.championName)}
            alt={championsById.get(me.championName)?.name ?? me.championName}
            className="h-8 w-8 shrink-0 rounded-full border border-[var(--color-accent-500)]"
          />
        )}
        <DeltaStat
          label="Niveau"
          mine={me.level}
          theirs={opponent.level}
          format={(value) => value.toFixed(0)}
        />
        <DeltaStat
          label="CS/min"
          mine={Number(csPerMinute(me.creepScore, gameTimeSeconds))}
          theirs={Number(csPerMinute(opponent.creepScore, gameTimeSeconds))}
          format={(value) => value.toFixed(1)}
        />
        <DeltaStat
          label="Objets"
          mine={me.itemCount}
          theirs={opponent.itemCount}
          format={(value) => value.toFixed(0)}
        />
        {version && (
          <RemoteIcon
            src={championIconUrl(version, opponent.championName)}
            alt={championsById.get(opponent.championName)?.name ?? opponent.championName}
            className="h-8 w-8 shrink-0 rounded-full border border-white/20"
          />
        )}
      </div>
    </div>
  );
}

export function OverlayPage() {
  useGamePhaseSync();
  const phase = useGamePhaseStore((state) => state.phase);
  const { data: snapshot } = useLiveGameSnapshot();
  const { data: version } = useLatestPatchVersion();
  const showGoldAndLevel = useOverlaySettingsStore((state) => state.showGoldAndLevel);
  const showObjectiveTimers = useOverlaySettingsStore((state) => state.showObjectiveTimers);
  const showContextualTip = useOverlaySettingsStore((state) => state.showContextualTip);
  const showScoreboard = useOverlaySettingsStore((state) => state.showScoreboard);
  const showLaneMatchup = useOverlaySettingsStore((state) => state.showLaneMatchup);
  const roster = useChampSelectRosterStore((state) => state.roster);
  const championsByKey = useChampionsByKey();

  const localTeam = snapshot?.players.find(
    (player) => player.summonerName === snapshot.activePlayerName,
  )?.team;
  const allies = snapshot?.players.filter((player) => player.team === localTeam) ?? [];
  const enemies = snapshot?.players.filter((player) => player.team !== localTeam) ?? [];

  const myRosterEntry = roster.find((participant) => participant.isLocalPlayer);
  const opponentRosterEntry = roster.find(
    (participant) =>
      !participant.isLocalPlayer &&
      participant.side === "enemy" &&
      participant.role === myRosterEntry?.role,
  );
  const opponentChampionDataDragonId = opponentRosterEntry
    ? championsByKey.get(String(opponentRosterEntry.championId))?.id
    : undefined;
  const myLive = snapshot?.players.find(
    (player) => player.summonerName === snapshot.activePlayerName,
  );
  const opponentLive = snapshot?.players.find(
    (player) => player.championName === opponentChampionDataDragonId,
  );

  return (
    <div
      data-tauri-drag-region
      className="flex h-screen w-screen flex-col overflow-hidden overlay-transparent p-2"
    >
      <div className="glass-panel flex flex-1 flex-col gap-2 rounded-2xl p-3 text-slate-100 shadow-lg">
        <div data-tauri-drag-region className="flex items-center justify-between">
          <span className="text-xs font-semibold uppercase tracking-wide text-[var(--color-accent-400)]">
            Wardstone
          </span>
          <button
            onClick={closeOverlay}
            className="rounded-md px-1.5 text-xs text-slate-400 hover:bg-white/10 hover:text-slate-100"
            aria-label="Fermer l'overlay"
          >
            ✕
          </button>
        </div>

        {!snapshot ? (
          <div className="flex flex-1 items-center justify-center text-sm text-slate-400">
            {GAME_PHASE_LABELS[phase]}
          </div>
        ) : (
          <>
            <div className="flex items-center justify-between text-sm">
              <span className="font-mono text-slate-200">
                ⏱ {formatGameClock(snapshot.gameTimeSeconds)}
              </span>
              {showGoldAndLevel && (
                <span className="text-slate-300">
                  💰 {Math.round(snapshot.activePlayerGold)} · Nv.{snapshot.activePlayerLevel}
                </span>
              )}
            </div>

            {showObjectiveTimers && (
              <div className="grid grid-cols-3 gap-2 text-center text-xs">
                <div className="rounded-lg bg-white/5 px-2 py-1.5">
                  <div className="text-slate-400">Dragon</div>
                  <div className="font-mono text-slate-100">
                    {formatTimer(snapshot.objectiveTimers.nextDragonSeconds)}
                  </div>
                </div>
                <div className="rounded-lg bg-white/5 px-2 py-1.5">
                  <div className="text-slate-400">Baron</div>
                  <div className="font-mono text-slate-100">
                    {formatTimer(snapshot.objectiveTimers.nextBaronSeconds)}
                  </div>
                </div>
                <div className="rounded-lg bg-white/5 px-2 py-1.5">
                  <div className="text-slate-400">Héraut</div>
                  <div className="font-mono text-slate-100">
                    {snapshot.objectiveTimers.heraldAvailable ? "Disponible" : "—"}
                  </div>
                </div>
              </div>
            )}

            {showContextualTip && (
              <p className="rounded-lg bg-[var(--color-accent-500)]/10 px-2 py-1.5 text-xs text-[var(--color-accent-400)]">
                {snapshot.contextualTip}
              </p>
            )}

            {showScoreboard && (
              <div className="grid grid-cols-2 gap-2 border-t border-white/10 pt-2">
                <div className="flex flex-col gap-0.5">
                  {allies.map((player) => (
                    <ScoreboardRow
                      key={player.summonerName}
                      player={player}
                      gameTimeSeconds={snapshot.gameTimeSeconds}
                      version={version}
                    />
                  ))}
                </div>
                <div className="flex flex-col gap-0.5">
                  {enemies.map((player) => (
                    <ScoreboardRow
                      key={player.summonerName}
                      player={player}
                      gameTimeSeconds={snapshot.gameTimeSeconds}
                      version={version}
                      reversed
                    />
                  ))}
                </div>
              </div>
            )}

            {showLaneMatchup && myLive && opponentLive && (
              <LaneMatchup
                me={myLive}
                opponent={opponentLive}
                gameTimeSeconds={snapshot.gameTimeSeconds}
                version={version}
              />
            )}
          </>
        )}
      </div>
    </div>
  );
}
