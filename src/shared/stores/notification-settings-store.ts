import { create } from "zustand";
import { persist } from "zustand/middleware";

export interface NotificationSettingsState {
  queueFound: boolean;
  championSelect: boolean;
  patchUpdate: boolean;
  winLoss: boolean;
  promotion: boolean;
  objectiveReached: boolean;
  toggle: (category: keyof Omit<NotificationSettingsState, "toggle">) => void;
}

export const useNotificationSettingsStore = create<NotificationSettingsState>()(
  persist(
    (set) => ({
      queueFound: true,
      championSelect: true,
      patchUpdate: true,
      winLoss: true,
      promotion: true,
      objectiveReached: true,
      toggle: (category) => set((state) => ({ [category]: !state[category] })),
    }),
    { name: "wardstone-notification-settings" },
  ),
);
