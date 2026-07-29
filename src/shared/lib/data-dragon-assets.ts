const DDRAGON_CDN = "https://ddragon.leagueoflegends.com";
const COMMUNITY_DRAGON_CDN = "https://raw.communitydragon.org";

/**
 * URLs vers les images statiques Data Dragon/Community Dragon, chargees
 * directement depuis le CDN public (whitelist deja en place dans la CSP,
 * voir `src-tauri/tauri.conf.json`). Les identifiants utilises (id Data
 * Dragon du champion, id numerique d'objet/sort, chemin d'icone de rune)
 * proviennent tous des donnees deja recuperees via `features/static-data`.
 */

export function championIconUrl(version: string, championDataDragonId: string): string {
  return `${DDRAGON_CDN}/cdn/${version}/img/champion/${championDataDragonId}.png`;
}

export function itemIconUrl(version: string, itemId: number | string): string {
  return `${DDRAGON_CDN}/cdn/${version}/img/item/${itemId}.png`;
}

export function summonerSpellIconUrl(version: string, spellDataDragonId: string): string {
  return `${DDRAGON_CDN}/cdn/${version}/img/spell/${spellDataDragonId}.png`;
}

/** `iconPath` est deja le chemin relatif complet fourni par `Rune.icon`. */
export function runeIconUrl(iconPath: string): string {
  return `${DDRAGON_CDN}/cdn/img/${iconPath}`;
}

export function profileIconUrl(version: string, profileIconId: number): string {
  return `${DDRAGON_CDN}/cdn/${version}/img/profileicon/${profileIconId}.png`;
}

/**
 * Blason de rang (Community Dragon, non fourni par Data Dragon). Absent pour
 * de tres recents ajouts de tier (ex: Emerald) selon la disponibilite des
 * assets en amont — a utiliser avec un `onError` masquant l'image.
 */
export function rankEmblemUrl(tier: string): string {
  return `${COMMUNITY_DRAGON_CDN}/latest/plugins/rcp-fe-lol-static-assets/global/default/images/ranked-mini-crests/${tier.toLowerCase()}.png`;
}
