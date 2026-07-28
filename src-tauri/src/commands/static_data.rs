use tauri::State;

use crate::infrastructure::data_dragon::dto::{ChampionDetail, ChampionSummary};
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
