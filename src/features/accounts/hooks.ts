import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query";
import { useEffect } from "react";

import { isTauriRuntime } from "@/shared/lib/tauri-bridge";
import { useActiveAccountStore } from "@/shared/stores/active-account-store";

import {
  deleteRiotApiKey,
  hasRiotApiKey,
  linkAccount,
  listAccounts,
  removeAccount,
  saveRiotApiKey,
  setPrimaryAccount,
} from "./api";
import type { AccountRecord } from "./types";

export function useHasRiotApiKey() {
  return useQuery({
    queryKey: ["riot-api-key", "status"],
    queryFn: hasRiotApiKey,
    enabled: isTauriRuntime(),
  });
}

export function useSaveRiotApiKey() {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: (apiKey: string) => saveRiotApiKey(apiKey),
    onSuccess: () => queryClient.invalidateQueries({ queryKey: ["riot-api-key"] }),
  });
}

export function useDeleteRiotApiKey() {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: deleteRiotApiKey,
    onSuccess: () => queryClient.invalidateQueries({ queryKey: ["riot-api-key"] }),
  });
}

export function useAccounts() {
  return useQuery({
    queryKey: ["accounts"],
    queryFn: listAccounts,
    enabled: isTauriRuntime(),
  });
}

export function useLinkAccount() {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: ({
      gameName,
      tagLine,
      platform,
    }: {
      gameName: string;
      tagLine: string;
      platform: string;
    }) => linkAccount(gameName, tagLine, platform),
    onSuccess: () => queryClient.invalidateQueries({ queryKey: ["accounts"] }),
  });
}

export function useRemoveAccount() {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: (puuid: string) => removeAccount(puuid),
    onSuccess: () => queryClient.invalidateQueries({ queryKey: ["accounts"] }),
  });
}

export function useSetPrimaryAccount() {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: (puuid: string) => setPrimaryAccount(puuid),
    onSuccess: () => queryClient.invalidateQueries({ queryKey: ["accounts"] }),
  });
}

/**
 * Maintient le compte actif synchronise avec la liste des comptes lies :
 * selectionne le compte principal (ou le premier) si aucun compte actif
 * n'est defini, et bascule automatiquement si le compte actif a ete
 * supprime.
 */
export function useSyncActiveAccount(accounts: AccountRecord[] | undefined) {
  const activeAccount = useActiveAccountStore((state) => state.activeAccount);
  const setActiveAccount = useActiveAccountStore((state) => state.setActiveAccount);

  useEffect(() => {
    if (!accounts) return;

    if (accounts.length === 0) {
      if (activeAccount !== null) setActiveAccount(null);
      return;
    }

    const stillLinked = activeAccount && accounts.some((a) => a.puuid === activeAccount.puuid);
    if (stillLinked) return;

    const fallback = accounts.find((a) => a.isPrimary) ?? accounts[0] ?? null;
    setActiveAccount(fallback);
  }, [accounts, activeAccount, setActiveAccount]);
}
