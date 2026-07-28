use chrono::Utc;
use tauri::State;

use crate::app_state::AppState;
use crate::infrastructure::db::accounts_repository::{self, AccountRecord};
use crate::infrastructure::db::Database;
use crate::infrastructure::riot_api::Platform;

/// Lie un nouveau compte Riot (Riot ID + region) au logiciel. Necessite
/// qu'une cle API Riot personnelle ait deja ete configuree
/// (voir `commands::riot_api_key`). Le premier compte lie devient
/// automatiquement le compte principal.
#[tauri::command]
pub async fn link_account(
    state: State<'_, AppState>,
    db: State<'_, Database>,
    game_name: String,
    tag_line: String,
    platform: String,
) -> Result<AccountRecord, String> {
    let riot_api = state
        .riot_api
        .read()
        .await
        .clone()
        .ok_or_else(|| "Aucune cle API Riot configuree.".to_string())?;

    let platform = Platform::from_str_loose(&platform)
        .ok_or_else(|| format!("Region inconnue: '{platform}'"))?;

    let account = riot_api
        .account_by_riot_id(platform.regional_route(), &game_name, &tag_line)
        .await
        .map_err(|err| err.to_string())?;

    let is_first_account = accounts_repository::list(&db.lock())
        .map_err(|err| err.to_string())?
        .is_empty();

    let record = AccountRecord {
        puuid: account.puuid,
        game_name: account.game_name.unwrap_or(game_name),
        tag_line: account.tag_line.unwrap_or(tag_line),
        platform: format!("{platform:?}").to_lowercase(),
        is_primary: is_first_account,
        linked_at: Utc::now().to_rfc3339(),
    };

    accounts_repository::upsert(&db.lock(), &record).map_err(|err| err.to_string())?;

    Ok(record)
}

#[tauri::command]
pub fn list_accounts(db: State<'_, Database>) -> Result<Vec<AccountRecord>, String> {
    accounts_repository::list(&db.lock()).map_err(|err| err.to_string())
}

#[tauri::command]
pub fn remove_account(db: State<'_, Database>, puuid: String) -> Result<(), String> {
    accounts_repository::delete(&db.lock(), &puuid).map_err(|err| err.to_string())
}

#[tauri::command]
pub fn set_primary_account(db: State<'_, Database>, puuid: String) -> Result<(), String> {
    accounts_repository::set_primary(&db.lock(), &puuid).map_err(|err| err.to_string())
}
