import type { ChampionInfo } from "@/features/static-data/types";

/**
 * Profil de composition d'equipe, derive uniquement des champs `info`
 * (attack/magic/defense, 0-10) deja fournis par Data Dragon pour chaque
 * champion — aucun appel API supplementaire, aucune donnee inventee.
 *
 * Volontairement limite au mix de dégâts et a la tankiness moyenne : un
 * modele d'engage/peel/teamfight/CC fiable necessiterait une base de
 * connaissance par sort (curatee a la main pour ~170 champions), que Data
 * Dragon n'expose pas de facon structuree. L'ajouter sans cette base
 * risquerait de desinformer (meme logique que la detection d'autofill,
 * volontairement non implementee — voir docs/ROADMAP.md).
 */

export type DamageLean = "AD" | "AP" | "Mixte";

const DAMAGE_LEAN_MARGIN = 2;

export function damageLean(info: ChampionInfo): DamageLean {
  const diff = info.attack - info.magic;
  if (diff >= DAMAGE_LEAN_MARGIN) return "AD";
  if (diff <= -DAMAGE_LEAN_MARGIN) return "AP";
  return "Mixte";
}

export interface CompositionProfile {
  adCount: number;
  apCount: number;
  mixedCount: number;
  averageTankiness: number;
}

export function computeCompositionProfile(
  championInfos: ChampionInfo[],
): CompositionProfile | null {
  if (championInfos.length === 0) return null;

  let adCount = 0;
  let apCount = 0;
  let mixedCount = 0;
  let defenseSum = 0;

  for (const info of championInfos) {
    const lean = damageLean(info);
    if (lean === "AD") adCount += 1;
    else if (lean === "AP") apCount += 1;
    else mixedCount += 1;
    defenseSum += info.defense;
  }

  return {
    adCount,
    apCount,
    mixedCount,
    averageTankiness: Math.round((defenseSum / championInfos.length) * 10) / 10,
  };
}

export function tankinessLabel(averageTankiness: number): string {
  if (averageTankiness >= 6) return "Élevée";
  if (averageTankiness >= 4) return "Moyenne";
  return "Faible";
}
