use tauri::State;

use crate::app_state::AppState;
use crate::infrastructure::lcu::champ_select::{self, LocalPlayerSelection};

/// Champion actuellement selectionne/verrouille par le joueur local en
/// champion select, avec le role assigne. `None` si le client n'est pas
/// joignable, pas en champion select, ou si aucun champion n'est encore
/// choisi.
#[tauri::command]
pub async fn get_current_champ_select_selection(
    state: State<'_, AppState>,
) -> Result<Option<LocalPlayerSelection>, String> {
    let connection_guard = state.lcu.connection.read().await;
    let Some(connection) = connection_guard.as_ref() else {
        return Ok(None);
    };

    champ_select::current_local_selection(&connection.client)
        .await
        .map_err(|err| err.to_string())
}
