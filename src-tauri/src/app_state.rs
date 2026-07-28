use std::sync::{Arc, Mutex};

use crate::domain::GamePhase;
use crate::infrastructure::lcu::LcuState;

/// Etat partage de l'application, injecte dans toutes les commandes Tauri
/// via `tauri::State`. Construit une seule fois au demarrage (voir `lib.rs`).
pub struct AppState {
    pub phase: Arc<Mutex<GamePhase>>,
    pub lcu: Arc<LcuState>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            phase: Arc::new(Mutex::new(GamePhase::ClientClosed)),
            lcu: Arc::new(LcuState::new()),
        }
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self::new()
    }
}
