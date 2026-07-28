use std::collections::HashMap;

use tauri::State;

use crate::infrastructure::data_dragon::dto::{
    ChampionDetail, ChampionSummary, ItemDetail, RuneTree, SummonerSpellDetail,
};
use crate::infrastructure::data_dragon::DataDragonClient;

/// Locale par defaut. Sera configurable via les parametres (Epic 9).
const DEFAULT_LOCALE: &str = "fr_FR";

#[tauri::command]
pub async fn get_latest_patch_version(
    data_dragon: State<'_, DataDragonClient>,
) -> Result<String, String> {
    data_dragon
        .latest_version()
        .await
        .map_err(|err| err.to_string())
}

#[tauri::command]
pub async fn get_champions(
    data_dragon: State<'_, DataDragonClient>,
) -> Result<Vec<ChampionSummary>, String> {
    let version = data_dragon
        .latest_version()
        .await
        .map_err(|err| err.to_string())?;
    let champions = data_dragon
        .champions(&version, DEFAULT_LOCALE)
        .await
        .map_err(|err| err.to_string())?;

    let mut list: Vec<ChampionSummary> = champions.data.into_values().collect();
    list.sort_by(|a, b| a.name.cmp(&b.name));
    Ok(list)
}

#[tauri::command]
pub async fn get_champion_detail(
    data_dragon: State<'_, DataDragonClient>,
    champion_id: String,
) -> Result<ChampionDetail, String> {
    let version = data_dragon
        .latest_version()
        .await
        .map_err(|err| err.to_string())?;
    let mut detail = data_dragon
        .champion_detail(&version, DEFAULT_LOCALE, &champion_id)
        .await
        .map_err(|err| err.to_string())?;

    detail
        .data
        .remove(&champion_id)
        .ok_or_else(|| format!("Champion '{champion_id}' introuvable"))
}

/// Objets, cle par identifiant numerique (tel qu'utilise par match-v5 et
/// l'assistant de champion select).
#[tauri::command]
pub async fn get_items(
    data_dragon: State<'_, DataDragonClient>,
) -> Result<HashMap<String, ItemDetail>, String> {
    let version = data_dragon
        .latest_version()
        .await
        .map_err(|err| err.to_string())?;
    let items = data_dragon
        .items(&version, DEFAULT_LOCALE)
        .await
        .map_err(|err| err.to_string())?;
    Ok(items.data)
}

#[tauri::command]
pub async fn get_runes(data_dragon: State<'_, DataDragonClient>) -> Result<Vec<RuneTree>, String> {
    let version = data_dragon
        .latest_version()
        .await
        .map_err(|err| err.to_string())?;
    data_dragon
        .runes(&version, DEFAULT_LOCALE)
        .await
        .map_err(|err| err.to_string())
}

#[tauri::command]
pub async fn get_summoner_spells(
    data_dragon: State<'_, DataDragonClient>,
) -> Result<HashMap<String, SummonerSpellDetail>, String> {
    let version = data_dragon
        .latest_version()
        .await
        .map_err(|err| err.to_string())?;
    let spells = data_dragon
        .summoner_spells(&version, DEFAULT_LOCALE)
        .await
        .map_err(|err| err.to_string())?;
    Ok(spells.data)
}
