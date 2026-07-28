import { useEffect, useRef } from "react";

import { listObjectivesWithProgress } from "@/features/objectives/api";
import { describeObjective } from "@/features/objectives/types";
import { getProfile } from "@/features/profile/api";
import { QUEUE_LABELS } from "@/features/profile/types";
import { useLatestPatchVersion } from "@/features/static-data/hooks";
import { isTauriRuntime } from "@/shared/lib/tauri-bridge";
import { useActiveAccountStore } from "@/shared/stores/active-account-store";
import { useGamePhaseStore } from "@/shared/stores/game-phase-store";
import { useRankSnapshotStore } from "@/shared/stores/rank-snapshot-store";
import type { GamePhase } from "@/shared/types/game-phase";

import { rankScore } from "./rank-order";
import { notify } from "./service";

const LAST_SEEN_PATCH_KEY = "wardstone-last-seen-patch";

/** Compare le profil courant au dernier releve connu pour detecter victoire,
 * defaite, promotion et nouveaux objectifs atteints. Appele une seule fois
 * par transition InProgress/WaitingForStats -> EndOfGame. Best-effort : ne
 * doit jamais interrompre le reste de l'application (compte non configure,
 * cle API absente, etc.). */
async function checkPostGameChanges(): Promise<void> {
  const activeAccount = useActiveAccountStore.getState().activeAccount;
  if (!isTauriRuntime() || !activeAccount) return;

  try {
    const profile = await getProfile(activeAccount.puuid, activeAccount.platform);
    const { snapshots, setSnapshot, achievedObjectiveIds, setAchievedObjectiveIds } =
      useRankSnapshotStore.getState();

    for (const entry of profile.leagueEntries) {
      const key = `${activeAccount.puuid}:${entry.queueType}`;
      const previous = snapshots[key];
      const queueLabel = QUEUE_LABELS[entry.queueType] ?? entry.queueType;

      if (previous) {
        if (entry.wins > previous.wins) {
          void notify(
            "winLoss",
            "Victoire !",
            `${queueLabel} — ${entry.tier} ${entry.rank}, ${entry.leaguePoints} LP.`,
          );
        } else if (entry.losses > previous.losses) {
          void notify(
            "winLoss",
            "Défaite",
            `${queueLabel} — ${entry.tier} ${entry.rank}, ${entry.leaguePoints} LP.`,
          );
        }

        if (rankScore(entry.tier, entry.rank) > rankScore(previous.tier, previous.rank)) {
          void notify(
            "promotion",
            "Promotion !",
            `Vous êtes maintenant ${entry.tier} ${entry.rank} en ${queueLabel}.`,
          );
        }
      }

      setSnapshot(key, {
        tier: entry.tier,
        rank: entry.rank,
        leaguePoints: entry.leaguePoints,
        wins: entry.wins,
        losses: entry.losses,
      });
    }

    const objectives = await listObjectivesWithProgress(
      activeAccount.puuid,
      activeAccount.platform,
    );
    const previouslyAchieved = new Set(achievedObjectiveIds[activeAccount.puuid] ?? []);
    const nowAchieved = objectives.filter((objective) => objective.progress.achieved);

    for (const objective of nowAchieved) {
      if (!previouslyAchieved.has(objective.id)) {
        void notify("objectiveReached", "Objectif atteint !", describeObjective(objective.kind));
      }
    }

    setAchievedObjectiveIds(
      activeAccount.puuid,
      nowAchieved.map((objective) => objective.id),
    );
  } catch {
    // Best-effort : voir commentaire de la fonction.
  }
}

/**
 * Orchestre les notifications OS natives : partie trouvee, entree en
 * selection de champion, victoire/defaite, promotion, objectif atteint et
 * nouveau patch. A monter une seule fois (fenetre principale) pour eviter
 * les doublons entre fenetre principale et overlay.
 */
export function useNotificationOrchestrator(): void {
  const phase = useGamePhaseStore((state) => state.phase);
  const { data: patchVersion } = useLatestPatchVersion();
  const previousPhase = useRef<GamePhase | null>(null);

  useEffect(() => {
    const previous = previousPhase.current;
    previousPhase.current = phase;
    if (previous === null || previous === phase) return;

    if (phase === "ReadyCheck") {
      void notify("queueFound", "Partie trouvée !", "Acceptez la partie dans le client League.");
    } else if (phase === "ChampSelect") {
      void notify(
        "championSelect",
        "Sélection de champion",
        "L'assistant Wardstone est disponible pour cette phase.",
      );
    } else if (
      phase === "EndOfGame" &&
      (previous === "InProgress" || previous === "WaitingForStats")
    ) {
      void checkPostGameChanges();
    }
  }, [phase]);

  useEffect(() => {
    if (!patchVersion || typeof window === "undefined") return;
    const lastSeen = window.localStorage.getItem(LAST_SEEN_PATCH_KEY);
    if (lastSeen && lastSeen !== patchVersion) {
      void notify(
        "patchUpdate",
        "Nouveau patch disponible",
        `League of Legends ${patchVersion} est arrivé.`,
      );
    }
    window.localStorage.setItem(LAST_SEEN_PATCH_KEY, patchVersion);
  }, [patchVersion]);
}
