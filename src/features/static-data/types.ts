export interface ChampionSummary {
  id: string;
  key: string;
  name: string;
  title: string;
  tags: string[];
  blurb: string;
}

export interface ChampionInfo {
  attack: number;
  defense: number;
  magic: number;
  difficulty: number;
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
