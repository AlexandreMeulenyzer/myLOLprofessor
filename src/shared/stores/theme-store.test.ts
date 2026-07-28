import { beforeEach, describe, expect, it } from "vitest";

import { useThemeStore } from "@/shared/stores/theme-store";

describe("useThemeStore", () => {
  beforeEach(() => {
    useThemeStore.setState({ theme: "dark", accent: "teal" });
  });

  it("defaults to the dark theme and teal accent", () => {
    const state = useThemeStore.getState();
    expect(state.theme).toBe("dark");
    expect(state.accent).toBe("teal");
  });

  it("updates the theme", () => {
    useThemeStore.getState().setTheme("oled");
    expect(useThemeStore.getState().theme).toBe("oled");
  });

  it("updates the accent color", () => {
    useThemeStore.getState().setAccent("violet");
    expect(useThemeStore.getState().accent).toBe("violet");
  });
});
