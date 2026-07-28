import { useMutation } from "@tanstack/react-query";

import { resolveSummoner } from "./api";

export function useResolveSummoner() {
  return useMutation({
    mutationFn: ({
      gameName,
      tagLine,
      platform,
    }: {
      gameName: string;
      tagLine: string;
      platform: string;
    }) => resolveSummoner(gameName, tagLine, platform),
  });
}
