import { create } from "zustand";
import { persist } from "zustand/middleware";

export interface RankSnapshotEntry {
  tier: string;
  rank: string;
  leaguePoints: number;
  wins: number;
  losses: number;
}

interface RankSnapshotState {
  /** Cle : `${puuid}:${queueType}`. Derniere valeur connue, pour detecter les
   * changements (victoire/defaite/promotion) entre deux fins de partie. */
  snapshots: Record<string, RankSnapshotEntry>;
  /** Cle : puuid. Identifiants des objectifs deja notifies comme atteints. */
  achievedObjectiveIds: Record<string, number[]>;
  setSnapshot: (key: string, entry: RankSnapshotEntry) => void;
  setAchievedObjectiveIds: (puuid: string, ids: number[]) => void;
}

export const useRankSnapshotStore = create<RankSnapshotState>()(
  persist(
    (set) => ({
      snapshots: {},
      achievedObjectiveIds: {},
      setSnapshot: (key, entry) =>
        set((state) => ({ snapshots: { ...state.snapshots, [key]: entry } })),
      setAchievedObjectiveIds: (puuid, ids) =>
        set((state) => ({
          achievedObjectiveIds: { ...state.achievedObjectiveIds, [puuid]: ids },
        })),
    }),
    { name: "wardstone-rank-snapshots" },
  ),
);
