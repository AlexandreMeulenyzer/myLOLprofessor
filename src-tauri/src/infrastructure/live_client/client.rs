use std::time::Duration;

use reqwest::Client;
use thiserror::Error;

use super::dto::AllGameData;

const LIVE_CLIENT_DATA_URL: &str = "https://127.0.0.1:2999/liveclientdata/allgamedata";

#[derive(Debug, Error)]
pub enum LiveClientError {
    #[error("aucune partie en cours (Live Client Data API indisponible)")]
    NotInGame,
    #[error("erreur reseau vers la Live Client Data API: {0}")]
    Network(#[from] reqwest::Error),
}

/// Client vers la Live Client Data API officielle de Riot
/// (`127.0.0.1:2999`), disponible uniquement pendant une partie en cours.
/// Comme le LCU, elle presente un certificat auto-signe strictement local.
pub struct LiveClientDataClient {
    http: Client,
}

impl LiveClientDataClient {
    pub fn new() -> Result<Self, LiveClientError> {
        let http = Client::builder()
            .danger_accept_invalid_certs(true)
            .timeout(Duration::from_secs(2))
            .build()?;
        Ok(Self { http })
    }

    pub async fn all_game_data(&self) -> Result<AllGameData, LiveClientError> {
        let response = self
            .http
            .get(LIVE_CLIENT_DATA_URL)
            .send()
            .await
            .map_err(|_| LiveClientError::NotInGame)?;

        if !response.status().is_success() {
            return Err(LiveClientError::NotInGame);
        }

        response
            .json::<AllGameData>()
            .await
            .map_err(LiveClientError::from)
    }
}

impl Default for LiveClientDataClient {
    fn default() -> Self {
        Self::new().expect("construction du client HTTP Live Client Data")
    }
}
