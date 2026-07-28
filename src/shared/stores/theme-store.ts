import { create } from "zustand";
import { persist } from "zustand/middleware";

export type ThemeMode = "dark" | "oled" | "light";
export type AccentColor = "teal" | "violet" | "amber" | "rose";

interface ThemeState {
  theme: ThemeMode;
  accent: AccentColor;
  setTheme: (theme: ThemeMode) => void;
  setAccent: (accent: AccentColor) => void;
}

export const useThemeStore = create<ThemeState>()(
  persist(
    (set) => ({
      theme: "dark",
      accent: "teal",
      setTheme: (theme) => set({ theme }),
      setAccent: (accent) => set({ accent }),
    }),
    { name: "wardstone-theme" },
  ),
);
