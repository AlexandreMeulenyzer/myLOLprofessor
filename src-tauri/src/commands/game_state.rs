use tauri::State;

use crate::app_state::AppState;
use crate::domain::GamePhase;

/// Retourne la phase de jeu actuelle (utile au chargement de l'UI, avant
/// meme le premier evenement `game-phase-changed`).
#[tauri::command]
pub fn get_game_phase(state: State<'_, AppState>) -> GamePhase {
    *state.phase.lock().expect("phase mutex poisoned")
}
