use std::sync::{Arc, Mutex};

use tokio::sync::RwLock;

use crate::domain::GamePhase;
use crate::infrastructure::lcu::LcuState;
use crate::infrastructure::riot_api::RiotApiClient;

/// Etat partage de l'application, injecte dans toutes les commandes Tauri
/// via `tauri::State`. Construit une seule fois au demarrage (voir `lib.rs`).
pub struct AppState {
    pub phase: Arc<Mutex<GamePhase>>,
    pub lcu: Arc<LcuState>,
    /// `None` tant que l'utilisateur n'a pas renseigne de cle API Riot
    /// valide (voir `commands::riot_account`).
    pub riot_api: RwLock<Option<Arc<RiotApiClient>>>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            phase: Arc::new(Mutex::new(GamePhase::ClientClosed)),
            lcu: Arc::new(LcuState::new()),
            riot_api: RwLock::new(None),
        }
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self::new()
    }
}
