import { describe, expect, it } from "vitest";

import { formatGameClock, formatTimer } from "./types";

describe("formatTimer", () => {
  it("shows 'Disponible' when the objective is up", () => {
    expect(formatTimer(0)).toBe("Disponible");
    expect(formatTimer(-5)).toBe("Disponible");
  });

  it("formats remaining seconds as mm:ss", () => {
    expect(formatTimer(90)).toBe("1:30");
    expect(formatTimer(65)).toBe("1:05");
  });
});

describe("formatGameClock", () => {
  it("formats the game clock as mm:ss", () => {
    expect(formatGameClock(0)).toBe("0:00");
    expect(formatGameClock(605)).toBe("10:05");
  });
});
