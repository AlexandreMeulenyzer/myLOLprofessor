import { useQueryClient } from "@tanstack/react-query";
import { useEffect } from "react";

import { isTauriRuntime, listen } from "@/shared/lib/tauri-bridge";

/**
 * Ecoute l'evenement `account-synced` emis par le backend (voir
 * `infrastructure::sync`) a chaque rafraichissement automatique d'un compte
 * lie (cycle periodique ou fin de partie) et invalide les caches React Query
 * concernes, pour que l'UI se mette a jour sans navigation ni action
 * manuelle de l'utilisateur.
 */
export function useAccountSyncListener(): void {
  const queryClient = useQueryClient();

  useEffect(() => {
    if (!isTauriRuntime()) {
      return;
    }

    const unlistenPromise = listen<string>("account-synced", () => {
      void queryClient.invalidateQueries({ queryKey: ["profile"] });
      void queryClient.invalidateQueries({ queryKey: ["match-history"] });
    });

    return () => {
      unlistenPromise.then((unlisten) => unlisten());
    };
  }, [queryClient]);
}
