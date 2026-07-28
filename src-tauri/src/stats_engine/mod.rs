use std::collections::HashMap;

use rusqlite::{params, Connection};
use serde::Serialize;

use crate::infrastructure::riot_api::dto::MatchParticipant;

/// Statistiques agregees d'un champion, pour un role et un patch donnes,
/// calculees localement a partir des matchs collectes via `match-v5` (voir
/// docs/ROADMAP.md — section "Pourquoi pas de scraping"). Aucune donnee
/// n'est empruntee a un service tiers.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ChampionRoleStats {
    pub champion_id: i64,
    pub role: String,
    pub patch: String,
    pub games: i64,
    pub wins: i64,
    pub winrate_percent: f64,
    /// `None` si le role ou le patch n'ont pas assez de donnees collectees
    /// pour estimer une pickrate fiable.
    pub pickrate_percent: Option<f64>,
    pub banrate_percent: Option<f64>,
    pub avg_kills: f64,
    pub avg_deaths: f64,
    pub avg_assists: f64,
    pub avg_game_duration_seconds: f64,
    pub avg_cs_per_min: f64,
    pub avg_gold_per_min: f64,
    /// (identifiant d'objet, nombre d'occurrences dans les 6 emplacements),
    /// trie par frequence decroissante.
    pub common_items: Vec<(i64, i64)>,
    /// (identifiant de sort d'invocateur, occurrences), trie par frequence.
    pub common_summoner_spells: Vec<(i64, i64)>,
    /// (identifiant de rune keystone, occurrences), trie par frequence.
    pub common_keystones: Vec<(i64, i64)>,
}

const TOP_N: usize = 6;

pub fn compute_champion_role_stats(
    conn: &Connection,
    champion_id: i64,
    role: &str,
    patch: &str,
) -> rusqlite::Result<Option<ChampionRoleStats>> {
    let mut statement = conn.prepare(
        "SELECT mp.stats_json, mp.win, m.duration_seconds
         FROM match_participants mp
         JOIN matches m ON m.match_id = mp.match_id
         WHERE mp.champion_id = ?1 AND mp.team_position = ?2 AND m.patch = ?3",
    )?;

    let rows: Vec<(String, bool, i64)> = statement
        .query_map(params![champion_id, role, patch], |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, i64>(1)? != 0,
                row.get::<_, i64>(2)?,
            ))
        })?
        .collect::<rusqlite::Result<_>>()?;

    if rows.is_empty() {
        return Ok(None);
    }

    let games = rows.len() as i64;
    let wins = rows.iter().filter(|(_, win, _)| *win).count() as i64;
    let total_duration: i64 = rows.iter().map(|(_, _, duration)| duration).sum();

    let mut total_kills = 0i64;
    let mut total_deaths = 0i64;
    let mut total_assists = 0i64;
    let mut total_cs_per_min = 0.0f64;
    let mut total_gold_per_min = 0.0f64;
    let mut item_counts: HashMap<i64, i64> = HashMap::new();
    let mut summoner_counts: HashMap<i64, i64> = HashMap::new();
    let mut keystone_counts: HashMap<i64, i64> = HashMap::new();

    for (stats_json, _, duration_seconds) in &rows {
        let Ok(participant) = serde_json::from_str::<MatchParticipant>(stats_json) else {
            continue;
        };

        total_kills += participant.kills;
        total_deaths += participant.deaths;
        total_assists += participant.assists;

        let minutes = (*duration_seconds as f64 / 60.0).max(1.0);
        total_cs_per_min += (participant.total_minions_killed + participant.neutral_minions_killed)
            as f64
            / minutes;
        total_gold_per_min += participant.gold_earned as f64 / minutes;

        for item_id in [
            participant.item0,
            participant.item1,
            participant.item2,
            participant.item3,
            participant.item4,
            participant.item5,
        ] {
            if item_id != 0 {
                *item_counts.entry(item_id).or_insert(0) += 1;
            }
        }

        *summoner_counts.entry(participant.summoner1_id).or_insert(0) += 1;
        *summoner_counts.entry(participant.summoner2_id).or_insert(0) += 1;

        if let Some(perks) = participant.perks {
            if let Some(primary_style) = perks.styles.first() {
                if let Some(keystone) = primary_style.selections.first() {
                    *keystone_counts.entry(keystone.perk).or_insert(0) += 1;
                }
            }
        }
    }

    let total_role_patch_games: i64 = conn.query_row(
        "SELECT COUNT(DISTINCT mp.match_id)
         FROM match_participants mp
         JOIN matches m ON m.match_id = mp.match_id
         WHERE mp.team_position = ?1 AND m.patch = ?2",
        params![role, patch],
        |row| row.get(0),
    )?;

    let total_patch_games: i64 = conn.query_row(
        "SELECT COUNT(*) FROM matches WHERE patch = ?1",
        params![patch],
        |row| row.get(0),
    )?;

    let banned_games = count_matches_with_banned_champion(conn, champion_id, patch)?;

    Ok(Some(ChampionRoleStats {
        champion_id,
        role: role.to_string(),
        patch: patch.to_string(),
        games,
        wins,
        winrate_percent: percent(wins, games),
        pickrate_percent: (total_role_patch_games > 0)
            .then(|| percent(games, total_role_patch_games)),
        banrate_percent: (total_patch_games > 0).then(|| percent(banned_games, total_patch_games)),
        avg_kills: average(total_kills, games),
        avg_deaths: average(total_deaths, games),
        avg_assists: average(total_assists, games),
        avg_game_duration_seconds: average(total_duration, games),
        avg_cs_per_min: round_one(total_cs_per_min / games as f64),
        avg_gold_per_min: round_one(total_gold_per_min / games as f64),
        common_items: top_n(item_counts),
        common_summoner_spells: top_n(summoner_counts),
        common_keystones: top_n(keystone_counts),
    }))
}

fn count_matches_with_banned_champion(
    conn: &Connection,
    champion_id: i64,
    patch: &str,
) -> rusqlite::Result<i64> {
    let mut statement = conn.prepare("SELECT bans_json FROM matches WHERE patch = ?1")?;
    let bans_per_match: Vec<String> = statement
        .query_map(params![patch], |row| row.get(0))?
        .collect::<rusqlite::Result<_>>()?;

    let count = bans_per_match
        .iter()
        .filter(|bans_json| {
            serde_json::from_str::<Vec<i64>>(bans_json)
                .map(|bans| bans.contains(&champion_id))
                .unwrap_or(false)
        })
        .count();

    Ok(count as i64)
}

fn percent(part: i64, total: i64) -> f64 {
    if total == 0 {
        return 0.0;
    }
    ((part as f64 / total as f64) * 1000.0).round() / 10.0
}

fn average(total: i64, games: i64) -> f64 {
    if games == 0 {
        return 0.0;
    }
    round_one(total as f64 / games as f64)
}

fn round_one(value: f64) -> f64 {
    (value * 10.0).round() / 10.0
}

fn top_n(counts: HashMap<i64, i64>) -> Vec<(i64, i64)> {
    let mut entries: Vec<(i64, i64)> = counts.into_iter().collect();
    entries.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(&b.0)));
    entries.truncate(TOP_N);
    entries
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::infrastructure::db::matches_repository::{self, MatchParticipantRecord};
    use crate::infrastructure::db::migrations;

    fn setup() -> Connection {
        let mut conn = Connection::open_in_memory().unwrap();
        migrations::run(&mut conn).unwrap();
        conn
    }

    fn participant_json(win: bool, kills: i64, item0: i64, keystone: i64) -> String {
        serde_json::json!({
            "puuid": "puuid-x",
            "championName": "Ahri",
            "championId": 103,
            "teamPosition": "MIDDLE",
            "teamId": 100,
            "win": win,
            "kills": kills,
            "deaths": 2,
            "assists": 5,
            "champLevel": 18,
            "goldEarned": 12000,
            "totalMinionsKilled": 180,
            "neutralMinionsKilled": 0,
            "visionScore": 20,
            "summoner1Id": 4,
            "summoner2Id": 14,
            "item0": item0,
            "item1": 0,
            "item2": 0,
            "item3": 0,
            "item4": 0,
            "item5": 0,
            "item6": 0,
            "perks": {
                "statPerks": {"offense": 1, "flex": 2, "defense": 3},
                "styles": [
                    {"description": "primaryStyle", "style": 8100, "selections": [{"perk": keystone, "var1": 0, "var2": 0, "var3": 0}]}
                ]
            }
        })
        .to_string()
    }

    fn insert_match(
        conn: &Connection,
        match_id: &str,
        win: bool,
        kills: i64,
        item0: i64,
        keystone: i64,
        bans: Vec<i64>,
    ) {
        matches_repository::upsert_match_participant(
            conn,
            &MatchParticipantRecord {
                match_id: match_id.to_string(),
                queue_id: 420,
                patch: "14.1".to_string(),
                played_at: "2026-01-01T00:00:00Z".to_string(),
                duration_seconds: 1800,
                banned_champion_ids: bans,
                puuid: "puuid-x".to_string(),
                champion: "Ahri".to_string(),
                champion_id: 103,
                team_position: "MIDDLE".to_string(),
                win,
                stats_json: participant_json(win, kills, item0, keystone),
            },
        )
        .unwrap();
    }

    #[test]
    fn returns_none_when_no_data_is_available() {
        let conn = setup();
        let result = compute_champion_role_stats(&conn, 103, "MIDDLE", "14.1").unwrap();
        assert!(result.is_none());
    }

    #[test]
    fn aggregates_winrate_kda_and_common_items() {
        let conn = setup();
        insert_match(&conn, "NA1_1", true, 10, 3020, 8214, vec![266]);
        insert_match(&conn, "NA1_2", false, 2, 3020, 8214, vec![]);
        insert_match(&conn, "NA1_3", true, 6, 6655, 8230, vec![103]);

        let stats = compute_champion_role_stats(&conn, 103, "MIDDLE", "14.1")
            .unwrap()
            .unwrap();

        assert_eq!(stats.games, 3);
        assert_eq!(stats.wins, 2);
        assert_eq!(stats.winrate_percent, 66.7);
        assert_eq!(stats.common_items[0], (3020, 2));
        assert_eq!(stats.common_keystones[0], (8214, 2));
        // banrate: 1 match sur 3 bannit le champion (id 103)
        assert_eq!(stats.banrate_percent, Some(33.3));
    }

    #[test]
    fn pickrate_reflects_share_of_role_patch_games() {
        let conn = setup();
        insert_match(&conn, "NA1_1", true, 5, 0, 8214, vec![]);

        // Un autre champion sur le meme role/patch, pour servir de denominateur.
        matches_repository::upsert_match_participant(
            &conn,
            &MatchParticipantRecord {
                match_id: "NA1_2".to_string(),
                queue_id: 420,
                patch: "14.1".to_string(),
                played_at: "2026-01-01T00:00:00Z".to_string(),
                duration_seconds: 1800,
                banned_champion_ids: vec![],
                puuid: "puuid-y".to_string(),
                champion: "Syndra".to_string(),
                champion_id: 134,
                team_position: "MIDDLE".to_string(),
                win: true,
                stats_json: participant_json(true, 5, 0, 8214),
            },
        )
        .unwrap();

        let stats = compute_champion_role_stats(&conn, 103, "MIDDLE", "14.1")
            .unwrap()
            .unwrap();
        assert_eq!(stats.pickrate_percent, Some(50.0));
    }
}
