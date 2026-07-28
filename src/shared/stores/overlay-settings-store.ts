import { create } from "zustand";
import { persist } from "zustand/middleware";

export interface OverlaySettingsState {
  showGoldAndLevel: boolean;
  showObjectiveTimers: boolean;
  showContextualTip: boolean;
  toggleWidget: (widget: keyof Omit<OverlaySettingsState, "toggleWidget">) => void;
}

export const useOverlaySettingsStore = create<OverlaySettingsState>()(
  persist(
    (set) => ({
      showGoldAndLevel: true,
      showObjectiveTimers: true,
      showContextualTip: true,
      toggleWidget: (widget) => set((state) => ({ [widget]: !state[widget] })),
    }),
    { name: "wardstone-overlay-settings" },
  ),
);
