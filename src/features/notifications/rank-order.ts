import { DIVISIONS, TIERS } from "@/features/objectives/types";

/** Score ordinal croissant (plus haut = meilleur rang). Utilise pour detecter
 * une promotion entre deux releves de rang ; ne distingue pas les LP au sein
 * d'une meme division/tier (suffisant pour une notification de promotion). */
export function rankScore(tier: string, rank: string): number {
  const tierIndex = TIERS.indexOf(tier as (typeof TIERS)[number]);
  const divisionIndex = DIVISIONS.indexOf(rank as (typeof DIVISIONS)[number]);
  const safeTier = tierIndex === -1 ? 0 : tierIndex;
  const safeDivision = divisionIndex === -1 ? 0 : divisionIndex;
  return safeTier * DIVISIONS.length + safeDivision;
}
