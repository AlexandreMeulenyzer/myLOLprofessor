use serde::Serialize;
use tauri::State;

use crate::app_state::AppState;
use crate::infrastructure::riot_api::Platform;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ResolvedSummoner {
    pub puuid: String,
    pub game_name: String,
    pub tag_line: String,
    pub platform: String,
    pub profile_icon_id: i64,
    pub summoner_level: i64,
}

/// Resout un Riot ID (nom#tag) en profil consultable, sans necessiter de
/// lier le compte localement. Utilise par la recherche globale et l'outil
/// de comparaison pour regarder le profil de n'importe quel joueur public.
#[tauri::command]
pub async fn resolve_summoner(
    state: State<'_, AppState>,
    game_name: String,
    tag_line: String,
    platform: String,
) -> Result<ResolvedSummoner, String> {
    let riot_api = state
        .riot_api
        .read()
        .await
        .clone()
        .ok_or_else(|| "Aucune cle API Riot configuree.".to_string())?;

    let platform_enum = Platform::from_str_loose(&platform)
        .ok_or_else(|| format!("Region inconnue: '{platform}'"))?;

    let account = riot_api
        .account_by_riot_id(platform_enum.regional_route(), &game_name, &tag_line)
        .await
        .map_err(|err| err.to_string())?;

    let summoner = riot_api
        .summoner_by_puuid(platform_enum, &account.puuid)
        .await
        .map_err(|err| err.to_string())?;

    Ok(ResolvedSummoner {
        puuid: account.puuid,
        game_name: account.game_name.unwrap_or(game_name),
        tag_line: account.tag_line.unwrap_or(tag_line),
        platform,
        profile_icon_id: summoner.profile_icon_id,
        summoner_level: summoner.summoner_level,
    })
}
