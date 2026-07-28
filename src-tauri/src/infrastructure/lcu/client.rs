use std::time::Duration;

use base64::{engine::general_purpose::STANDARD, Engine};
use reqwest::Client;
use serde::de::DeserializeOwned;
use thiserror::Error;

use super::process_discovery::LcuCredentials;

/// Espace d'erreur complet des echanges avec le LCU. Certains variants ne
/// sont pas encore construits par le watcher (Epic 1) mais le seront par les
/// commandes des Epics 3/4 (session de champion select, scan des joueurs).
#[derive(Debug, Error)]
#[allow(dead_code)]
pub enum LcuError {
    #[error("le client League of Legends n'est pas lance")]
    NotRunning,
    #[error("erreur reseau vers le client LCU: {0}")]
    Network(#[from] reqwest::Error),
    #[error("reponse LCU invalide: {0}")]
    InvalidResponse(String),
}

/// Client REST vers l'API locale du League Client Update (LCU). Le
/// certificat presente par le client est auto-signe : la validation TLS
/// standard est desactivee volontairement, uniquement pour cette connexion
/// strictement locale (127.0.0.1).
#[derive(Clone)]
pub struct LcuClient {
    http: Client,
    base_url: String,
    auth_header: String,
}

impl LcuClient {
    pub fn new(credentials: &LcuCredentials) -> Result<Self, LcuError> {
        let http = Client::builder()
            .danger_accept_invalid_certs(true)
            .timeout(Duration::from_secs(4))
            .build()?;

        let auth_header = format!(
            "Basic {}",
            STANDARD.encode(format!("riot:{}", credentials.auth_token))
        );

        Ok(Self {
            http,
            base_url: format!("https://127.0.0.1:{}", credentials.port),
            auth_header,
        })
    }

    pub async fn get_json<T: DeserializeOwned>(&self, path: &str) -> Result<T, LcuError> {
        let response = self
            .http
            .get(format!("{}{}", self.base_url, path))
            .header("Authorization", &self.auth_header)
            .send()
            .await?
            .error_for_status()?;

        response.json::<T>().await.map_err(LcuError::Network)
    }

    /// Retourne la valeur brute de `/lol-gameflow/v1/gameflow-phase`
    /// (ex: "None", "ChampSelect", "InProgress"...).
    pub async fn gameflow_phase(&self) -> Result<String, LcuError> {
        self.get_json::<String>("/lol-gameflow/v1/gameflow-phase")
            .await
    }

    #[allow(dead_code)]
    pub fn port(&self) -> Option<u16> {
        self.base_url
            .rsplit(':')
            .next()
            .and_then(|p| p.parse().ok())
    }

    #[allow(dead_code)]
    pub fn auth_header(&self) -> &str {
        &self.auth_header
    }
}
