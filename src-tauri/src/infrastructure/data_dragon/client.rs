use std::path::{Path, PathBuf};
use std::time::{Duration, SystemTime};

use reqwest::Client;
use serde::de::DeserializeOwned;
use serde::Serialize;
use thiserror::Error;

use super::dto::{ChampionDetailData, ChampionsData, ItemsData, RuneTree, SummonerSpellsData};

const DDRAGON_CDN: &str = "https://ddragon.leagueoflegends.com";
/// Duree de vie du cache de la liste des versions : suffisamment courte
/// pour detecter un nouveau patch rapidement, assez longue pour ne pas
/// spammer Data Dragon a chaque ouverture de l'app.
const VERSIONS_TTL: Duration = Duration::from_secs(3600);

#[derive(Debug, Error)]
pub enum DataDragonError {
    #[error("erreur reseau vers Data Dragon: {0}")]
    Network(#[from] reqwest::Error),
    #[error("erreur de lecture/ecriture du cache local: {0}")]
    Io(#[from] std::io::Error),
    #[error("reponse Data Dragon invalide: {0}")]
    InvalidResponse(#[from] serde_json::Error),
    #[error("aucune version de patch disponible")]
    NoVersionAvailable,
}

/// Client vers Data Dragon (donnees statiques officielles : champions,
/// objets, runes, sorts d'invocateur). Les donnees par version sont
/// immuables une fois publiees et mises en cache disque indefiniment ; la
/// liste des versions est revalidee toutes les heures pour detecter un
/// nouveau patch.
pub struct DataDragonClient {
    http: Client,
    cache_dir: PathBuf,
}

impl DataDragonClient {
    pub fn new(cache_dir: PathBuf) -> Self {
        Self {
            http: Client::new(),
            cache_dir,
        }
    }

    pub async fn latest_version(&self) -> Result<String, DataDragonError> {
        let versions = self.versions().await?;
        versions
            .into_iter()
            .next()
            .ok_or(DataDragonError::NoVersionAvailable)
    }

    pub async fn versions(&self) -> Result<Vec<String>, DataDragonError> {
        let cache_path = self.cache_dir.join("versions.json");

        if let Some(cached) = read_if_fresh::<Vec<String>>(&cache_path, VERSIONS_TTL).await {
            return Ok(cached);
        }

        let versions: Vec<String> = self
            .http
            .get(format!("{DDRAGON_CDN}/api/versions.json"))
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?;

        write_cache(&cache_path, &versions).await?;
        Ok(versions)
    }

    pub async fn champions(
        &self,
        version: &str,
        locale: &str,
    ) -> Result<ChampionsData, DataDragonError> {
        self.fetch_immutable(
            &format!("cdn/{version}/data/{locale}/champion.json"),
            &format!("{version}/{locale}/champion.json"),
        )
        .await
    }

    pub async fn champion_detail(
        &self,
        version: &str,
        locale: &str,
        champion_id: &str,
    ) -> Result<ChampionDetailData, DataDragonError> {
        self.fetch_immutable(
            &format!("cdn/{version}/data/{locale}/champion/{champion_id}.json"),
            &format!("{version}/{locale}/champion-{champion_id}.json"),
        )
        .await
    }

    /// Consomme par l'Assistant de Champion Select (Epic 3).
    #[allow(dead_code)]
    pub async fn items(&self, version: &str, locale: &str) -> Result<ItemsData, DataDragonError> {
        self.fetch_immutable(
            &format!("cdn/{version}/data/{locale}/item.json"),
            &format!("{version}/{locale}/item.json"),
        )
        .await
    }

    /// Consomme par l'Assistant de Champion Select (Epic 3).
    #[allow(dead_code)]
    pub async fn runes(
        &self,
        version: &str,
        locale: &str,
    ) -> Result<Vec<RuneTree>, DataDragonError> {
        self.fetch_immutable(
            &format!("cdn/{version}/data/{locale}/runesReforged.json"),
            &format!("{version}/{locale}/runesReforged.json"),
        )
        .await
    }

    /// Consomme par l'Assistant de Champion Select (Epic 3).
    #[allow(dead_code)]
    pub async fn summoner_spells(
        &self,
        version: &str,
        locale: &str,
    ) -> Result<SummonerSpellsData, DataDragonError> {
        self.fetch_immutable(
            &format!("cdn/{version}/data/{locale}/summoner.json"),
            &format!("{version}/{locale}/summoner.json"),
        )
        .await
    }

    async fn fetch_immutable<T: DeserializeOwned + Serialize>(
        &self,
        remote_path: &str,
        cache_relative_path: &str,
    ) -> Result<T, DataDragonError> {
        let cache_path = self.cache_dir.join(cache_relative_path);

        if let Some(cached) = read_cache::<T>(&cache_path).await {
            return Ok(cached);
        }

        let value: T = self
            .http
            .get(format!("{DDRAGON_CDN}/{remote_path}"))
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?;

        write_cache(&cache_path, &value).await?;
        Ok(value)
    }
}

async fn read_cache<T: DeserializeOwned>(path: &Path) -> Option<T> {
    let bytes = tokio::fs::read(path).await.ok()?;
    serde_json::from_slice(&bytes).ok()
}

async fn read_if_fresh<T: DeserializeOwned>(path: &Path, ttl: Duration) -> Option<T> {
    let metadata = tokio::fs::metadata(path).await.ok()?;
    let modified = metadata.modified().ok()?;
    let age = SystemTime::now().duration_since(modified).ok()?;
    if age > ttl {
        return None;
    }
    read_cache(path).await
}

async fn write_cache<T: Serialize>(path: &Path, value: &T) -> Result<(), DataDragonError> {
    if let Some(parent) = path.parent() {
        tokio::fs::create_dir_all(parent).await?;
    }
    let bytes = serde_json::to_vec(value)?;
    tokio::fs::write(path, bytes).await?;
    Ok(())
}
