import { describe, expect, it } from "vitest";

import { queueName } from "./types";

describe("queueName", () => {
  it("returns a known queue label", () => {
    expect(queueName(420)).toBe("Classée Solo/Duo");
    expect(queueName(450)).toBe("ARAM");
  });

  it("falls back to a generic label for unknown queues", () => {
    expect(queueName(9999)).toBe("File #9999");
  });
});
