use std::sync::Arc;

use tauri::State;

use crate::app_state::AppState;
use crate::infrastructure::riot_api::RiotApiClient;
use crate::infrastructure::secure_storage;

/// Enregistre la cle API Riot personnelle de l'utilisateur dans le
/// trousseau securise du systeme et l'active immediatement pour la session
/// en cours (aucun redemarrage necessaire).
#[tauri::command]
pub async fn save_riot_api_key(state: State<'_, AppState>, api_key: String) -> Result<(), String> {
    let api_key = api_key.trim().to_string();
    if api_key.is_empty() {
        return Err("La cle API ne peut pas etre vide.".to_string());
    }

    secure_storage::save_riot_api_key(&api_key).map_err(|err| err.to_string())?;

    let client = RiotApiClient::new(api_key).map_err(|err| err.to_string())?;
    *state.riot_api.write().await = Some(Arc::new(client));

    Ok(())
}

/// Indique si une cle API Riot est configuree pour la session en cours.
#[tauri::command]
pub async fn has_riot_api_key(state: State<'_, AppState>) -> Result<bool, String> {
    Ok(state.riot_api.read().await.is_some())
}

/// Supprime la cle API Riot du trousseau systeme et desactive les appels
/// Riot API pour la session en cours.
#[tauri::command]
pub async fn delete_riot_api_key(state: State<'_, AppState>) -> Result<(), String> {
    secure_storage::delete_riot_api_key().map_err(|err| err.to_string())?;
    *state.riot_api.write().await = None;
    Ok(())
}
