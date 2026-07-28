pub mod client;
pub mod dto;

#[allow(unused_imports)] // erreur publique du module, consommee des que les
// commandes de l'Epic 3 propagent les erreurs Data Dragon au frontend.
pub use client::{DataDragonClient, DataDragonError};
