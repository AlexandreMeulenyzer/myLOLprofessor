use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChampionsData {
    pub version: String,
    pub data: HashMap<String, ChampionSummary>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChampionSummary {
    pub id: String,
    pub key: String,
    pub name: String,
    pub title: String,
    pub tags: Vec<String>,
    pub blurb: String,
    /// Deja present dans le champion.json "en masse" (pas seulement dans le
    /// detail par champion) — utilise par l'analyse de composition d'equipe
    /// (mix AD/AP, tankiness) sans appel API supplementaire.
    pub info: ChampionInfo,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChampionDetailData {
    pub data: HashMap<String, ChampionDetail>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChampionDetail {
    pub id: String,
    pub key: String,
    pub name: String,
    pub title: String,
    pub tags: Vec<String>,
    pub partype: String,
    pub info: ChampionInfo,
    pub lore: String,
    pub allytips: Vec<String>,
    pub enemytips: Vec<String>,
    pub passive: ChampionPassive,
    pub spells: Vec<ChampionSpell>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChampionInfo {
    pub attack: i64,
    pub defense: i64,
    pub magic: i64,
    pub difficulty: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChampionPassive {
    pub name: String,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChampionSpell {
    pub id: String,
    pub name: String,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemsData {
    pub version: String,
    pub data: HashMap<String, ItemDetail>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemDetail {
    pub name: String,
    #[serde(default)]
    pub plaintext: String,
    #[serde(default)]
    pub description: String,
    pub gold: ItemGold,
    #[serde(default)]
    pub tags: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemGold {
    pub base: i64,
    pub total: i64,
    pub sell: i64,
    pub purchasable: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuneTree {
    pub id: i64,
    pub key: String,
    pub icon: String,
    pub name: String,
    pub slots: Vec<RuneSlot>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuneSlot {
    pub runes: Vec<Rune>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Rune {
    pub id: i64,
    pub key: String,
    pub icon: String,
    pub name: String,
    #[serde(rename = "shortDesc")]
    pub short_desc: String,
    #[serde(rename = "longDesc")]
    pub long_desc: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SummonerSpellsData {
    pub version: String,
    pub data: HashMap<String, SummonerSpellDetail>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SummonerSpellDetail {
    pub id: String,
    pub name: String,
    pub description: String,
    /// Identifiant numerique (ex: "4" pour Flash) utilise par match-v5
    /// (`summoner1Id`/`summoner2Id`).
    pub key: String,
}
