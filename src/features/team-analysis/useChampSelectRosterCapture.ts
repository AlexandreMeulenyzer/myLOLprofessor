import { useEffect } from "react";

import { useActiveAccountStore } from "@/shared/stores/active-account-store";
import { useChampSelectRosterStore } from "@/shared/stores/champ-select-roster-store";

import { useChampSelectTeamAnalysis } from "./hooks";

/**
 * Capture le roster d'analyse d'equipe des qu'il est disponible en champion
 * select, independamment de la page affichee (voir `AppShell`) : l'API LCU
 * de champion select ne sera plus interrogeable une fois la partie lancee,
 * ce roster est donc la seule source pour reconstituer les roles/camps
 * pendant la partie (ex: face-a-face de lane dans l'overlay).
 */
export function useChampSelectRosterCapture(): void {
  const activeAccount = useActiveAccountStore((state) => state.activeAccount);
  const { data } = useChampSelectTeamAnalysis(activeAccount?.platform);
  const setRoster = useChampSelectRosterStore((state) => state.setRoster);

  useEffect(() => {
    if (data && data.length > 0) {
      setRoster(data);
    }
  }, [data, setRoster]);
}
