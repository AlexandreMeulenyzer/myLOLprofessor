import { create } from "zustand";
import { persist } from "zustand/middleware";

import type { AccountRecord } from "@/features/accounts/types";

interface ActiveAccountState {
  activeAccount: AccountRecord | null;
  setActiveAccount: (account: AccountRecord | null) => void;
}

export const useActiveAccountStore = create<ActiveAccountState>()(
  persist(
    (set) => ({
      activeAccount: null,
      setActiveAccount: (account) => set({ activeAccount: account }),
    }),
    { name: "wardstone-active-account" },
  ),
);
