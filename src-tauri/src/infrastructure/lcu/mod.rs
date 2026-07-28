pub mod champ_select;
pub mod client;
pub mod process_discovery;
pub mod watcher;
pub mod websocket;

pub use client::LcuClient;
pub use process_discovery::LcuCredentials;

use tokio::sync::RwLock;

/// Connexion LCU active partagee entre le watcher et les commandes Tauri
/// (analyse d'equipe, session de champion select...).
pub struct LcuConnection {
    #[allow(dead_code)] // consomme par l'analyse d'equipe (Epic 4)
    pub credentials: LcuCredentials,
    pub client: LcuClient,
}

/// Etat partage : `None` lorsque le client League of Legends n'est pas
/// lance ou n'est pas encore joignable.
#[derive(Default)]
pub struct LcuState {
    pub connection: RwLock<Option<LcuConnection>>,
}

impl LcuState {
    pub fn new() -> Self {
        Self::default()
    }
}
