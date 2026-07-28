pub mod client;
pub mod process_discovery;
pub mod watcher;
pub mod websocket;

pub use client::LcuClient;
pub use process_discovery::LcuCredentials;

use tokio::sync::RwLock;

/// Connexion LCU active partagee entre le watcher et les commandes Tauri
/// (analyse d'equipe, session de champion select...). Les champs sont
/// consommes par les commandes ajoutees dans les Epics 3/4 (session de
/// champion select, scan des joueurs).
#[allow(dead_code)]
pub struct LcuConnection {
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
