import { describe, expect, it } from "vitest";

import { computeCompositionProfile, damageLean, tankinessLabel } from "./composition";

function info(attack: number, magic: number, defense: number) {
  return { attack, magic, defense, difficulty: 5 };
}

describe("damageLean", () => {
  it("classifies clearly AD champions", () => {
    expect(damageLean(info(9, 2, 4))).toBe("AD");
  });

  it("classifies clearly AP champions", () => {
    expect(damageLean(info(2, 9, 4))).toBe("AP");
  });

  it("classifies close attack/magic as mixed", () => {
    expect(damageLean(info(6, 5, 4))).toBe("Mixte");
  });
});

describe("computeCompositionProfile", () => {
  it("returns null for an empty roster", () => {
    expect(computeCompositionProfile([])).toBeNull();
  });

  it("aggregates damage lean counts and average tankiness", () => {
    const profile = computeCompositionProfile([info(9, 2, 2), info(2, 9, 6), info(5, 5, 8)]);

    expect(profile).toEqual({
      adCount: 1,
      apCount: 1,
      mixedCount: 1,
      averageTankiness: 5.3,
    });
  });
});

describe("tankinessLabel", () => {
  it("labels low tankiness", () => {
    expect(tankinessLabel(2)).toBe("Faible");
  });

  it("labels mid tankiness", () => {
    expect(tankinessLabel(4)).toBe("Moyenne");
  });

  it("labels high tankiness", () => {
    expect(tankinessLabel(7)).toBe("Élevée");
  });
});
