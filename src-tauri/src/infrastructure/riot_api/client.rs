use std::time::Duration;

use percent_encoding::{utf8_percent_encode, NON_ALPHANUMERIC};
use reqwest::{Client, StatusCode};
use serde::de::DeserializeOwned;
use thiserror::Error;

use super::dto::{
    AccountDto, ChampionMasteryDto, CurrentGameInfo, LeagueEntryDto, MatchDto, SummonerDto,
};
use super::rate_limiter::RateLimiter;
use super::regions::{Platform, RegionalRoute};

const MAX_RETRIES: u32 = 3;

#[derive(Debug, Error)]
pub enum RiotApiError {
    #[error("erreur reseau vers l'API Riot: {0}")]
    Network(#[from] reqwest::Error),
    #[error("ressource introuvable (404)")]
    NotFound,
    #[error("cle API Riot invalide ou expiree")]
    InvalidApiKey,
    #[error("limite de requetes Riot API depassee malgre les tentatives")]
    RateLimited,
    #[error("reponse HTTP inattendue de l'API Riot: {0}")]
    Http(u16),
}

/// Client HTTP vers l'API officielle Riot Games. Gere le routing
/// regional/plateforme, le rate limiting cote client et les tentatives avec
/// backoff exponentiel sur les erreurs transitoires (429/5xx).
pub struct RiotApiClient {
    http: Client,
    api_key: String,
    limiter: RateLimiter,
}

impl RiotApiClient {
    pub fn new(api_key: String) -> Result<Self, RiotApiError> {
        let http = Client::builder().timeout(Duration::from_secs(10)).build()?;
        Ok(Self {
            http,
            api_key,
            limiter: RateLimiter::development_defaults(),
        })
    }

    async fn get_json<T: DeserializeOwned>(
        &self,
        host: &str,
        path: &str,
    ) -> Result<T, RiotApiError> {
        let mut attempt = 0u32;

        loop {
            self.limiter.acquire().await;

            let response = self
                .http
                .get(format!("https://{host}{path}"))
                .header("X-Riot-Token", &self.api_key)
                .send()
                .await?;

            let status = response.status();

            match status {
                StatusCode::OK => return response.json::<T>().await.map_err(RiotApiError::from),
                StatusCode::NOT_FOUND => return Err(RiotApiError::NotFound),
                StatusCode::UNAUTHORIZED | StatusCode::FORBIDDEN => {
                    return Err(RiotApiError::InvalidApiKey)
                }
                StatusCode::TOO_MANY_REQUESTS => {
                    if attempt >= MAX_RETRIES {
                        return Err(RiotApiError::RateLimited);
                    }
                    let retry_after = response
                        .headers()
                        .get("Retry-After")
                        .and_then(|value| value.to_str().ok())
                        .and_then(|value| value.parse::<u64>().ok())
                        .unwrap_or(1);
                    tokio::time::sleep(Duration::from_secs(retry_after)).await;
                }
                status if status.is_server_error() => {
                    if attempt >= MAX_RETRIES {
                        return Err(RiotApiError::Http(status.as_u16()));
                    }
                    tokio::time::sleep(backoff_delay(attempt)).await;
                }
                status => return Err(RiotApiError::Http(status.as_u16())),
            }

            attempt += 1;
        }
    }

    pub async fn account_by_puuid(
        &self,
        route: RegionalRoute,
        puuid: &str,
    ) -> Result<AccountDto, RiotApiError> {
        self.get_json(
            route.host(),
            &format!(
                "/riot/account/v1/accounts/by-puuid/{}",
                encode_path_segment(puuid)
            ),
        )
        .await
    }

    pub async fn account_by_riot_id(
        &self,
        route: RegionalRoute,
        game_name: &str,
        tag_line: &str,
    ) -> Result<AccountDto, RiotApiError> {
        let path = format!(
            "/riot/account/v1/accounts/by-riot-id/{}/{}",
            encode_path_segment(game_name),
            encode_path_segment(tag_line)
        );
        self.get_json(route.host(), &path).await
    }

    pub async fn summoner_by_puuid(
        &self,
        platform: Platform,
        puuid: &str,
    ) -> Result<SummonerDto, RiotApiError> {
        self.get_json(
            platform.host(),
            &format!(
                "/lol/summoner/v4/summoners/by-puuid/{}",
                encode_path_segment(puuid)
            ),
        )
        .await
    }

    pub async fn league_entries_by_puuid(
        &self,
        platform: Platform,
        puuid: &str,
    ) -> Result<Vec<LeagueEntryDto>, RiotApiError> {
        self.get_json(
            platform.host(),
            &format!(
                "/lol/league/v4/entries/by-puuid/{}",
                encode_path_segment(puuid)
            ),
        )
        .await
    }

    pub async fn champion_masteries_by_puuid(
        &self,
        platform: Platform,
        puuid: &str,
    ) -> Result<Vec<ChampionMasteryDto>, RiotApiError> {
        self.get_json(
            platform.host(),
            &format!(
                "/lol/champion-mastery/v4/champion-masteries/by-puuid/{}",
                encode_path_segment(puuid)
            ),
        )
        .await
    }

    pub async fn match_ids_by_puuid(
        &self,
        route: RegionalRoute,
        puuid: &str,
        count: u32,
    ) -> Result<Vec<String>, RiotApiError> {
        self.get_json(
            route.host(),
            &format!(
                "/lol/match/v5/matches/by-puuid/{}/ids?count={}",
                encode_path_segment(puuid),
                count.min(100)
            ),
        )
        .await
    }

    pub async fn match_by_id(
        &self,
        route: RegionalRoute,
        match_id: &str,
    ) -> Result<MatchDto, RiotApiError> {
        self.get_json(
            route.host(),
            &format!("/lol/match/v5/matches/{}", encode_path_segment(match_id)),
        )
        .await
    }

    /// `None` si le joueur n'est actuellement pas en partie (404 attendu et
    /// non exceptionnel dans ce cas precis). Consomme par l'analyse d'equipe
    /// et l'overlay in-game (Epics 4/5).
    #[allow(dead_code)]
    pub async fn active_game_by_puuid(
        &self,
        platform: Platform,
        puuid: &str,
    ) -> Result<Option<CurrentGameInfo>, RiotApiError> {
        match self
            .get_json(
                platform.host(),
                &format!(
                    "/lol/spectator/v5/active-games/by-summoner/{}",
                    encode_path_segment(puuid)
                ),
            )
            .await
        {
            Ok(info) => Ok(Some(info)),
            Err(RiotApiError::NotFound) => Ok(None),
            Err(err) => Err(err),
        }
    }
}

fn encode_path_segment(value: &str) -> String {
    utf8_percent_encode(value, NON_ALPHANUMERIC).to_string()
}

fn backoff_delay(attempt: u32) -> Duration {
    Duration::from_millis(200 * 2u64.pow(attempt))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encodes_special_characters_in_riot_ids() {
        assert_eq!(encode_path_segment("Faker#KR1"), "Faker%23KR1");
        assert_eq!(
            encode_path_segment("Été Été"),
            "%C3%89t%C3%A9%20%C3%89t%C3%A9"
        );
    }

    #[test]
    fn backoff_grows_exponentially() {
        assert_eq!(backoff_delay(0), Duration::from_millis(200));
        assert_eq!(backoff_delay(1), Duration::from_millis(400));
        assert_eq!(backoff_delay(2), Duration::from_millis(800));
    }
}
