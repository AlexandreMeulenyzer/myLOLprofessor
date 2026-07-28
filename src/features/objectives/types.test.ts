import { describe, expect, it } from "vitest";

import { describeObjective } from "./types";

describe("describeObjective", () => {
  it("describes a reachRank objective", () => {
    expect(
      describeObjective({
        type: "reachRank",
        queueType: "RANKED_SOLO_5x5",
        tier: "GOLD",
        rank: "II",
      }),
    ).toBe("Atteindre GOLD II");
  });

  it("describes a winrateTarget objective", () => {
    expect(
      describeObjective({
        type: "winrateTarget",
        queueType: "RANKED_SOLO_5x5",
        percent: 60,
        minGames: 20,
      }),
    ).toBe("60% de winrate (min. 20 parties)");
  });

  it("describes a gamesPlayed objective", () => {
    expect(describeObjective({ type: "gamesPlayed", count: 50 })).toBe("Jouer 50 parties");
  });
});
