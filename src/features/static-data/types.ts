export interface ChampionInfo {
  attack: number;
  defense: number;
  magic: number;
  difficulty: number;
}

export interface ChampionSummary {
  id: string;
  key: string;
  name: string;
  title: string;
  tags: string[];
  blurb: string;
  info: ChampionInfo;
}

export interface ChampionPassive {
  name: string;
  description: string;
}

export interface ChampionSpell {
  id: string;
  name: string;
  description: string;
}

export interface ChampionDetail {
  id: string;
  key: string;
  name: string;
  title: string;
  tags: string[];
  partype: string;
  info: ChampionInfo;
  lore: string;
  allytips: string[];
  enemytips: string[];
  passive: ChampionPassive;
  spells: ChampionSpell[];
}

export interface ItemGold {
  base: number;
  total: number;
  sell: number;
  purchasable: boolean;
}

export interface ItemDetail {
  name: string;
  plaintext: string;
  description: string;
  gold: ItemGold;
  tags: string[];
}

export interface Rune {
  id: number;
  key: string;
  icon: string;
  name: string;
  shortDesc: string;
  longDesc: string;
}

export interface RuneSlot {
  runes: Rune[];
}

export interface RuneTree {
  id: number;
  key: string;
  icon: string;
  name: string;
  slots: RuneSlot[];
}

export interface SummonerSpellDetail {
  id: string;
  name: string;
  description: string;
  key: string;
}
