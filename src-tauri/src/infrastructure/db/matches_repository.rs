use rusqlite::{params, Connection, OptionalExtension};
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MatchParticipantRecord {
    pub match_id: String,
    pub queue_id: i64,
    pub patch: String,
    pub played_at: String,
    pub duration_seconds: i64,
    /// Champions bannis dans ce match (identifiants numeriques, toutes
    /// equipes confondues) — alimente le calcul de banrate du stats engine.
    pub banned_champion_ids: Vec<i64>,
    pub puuid: String,
    pub champion: String,
    pub champion_id: i64,
    pub team_position: String,
    pub win: bool,
    /// Statistiques completes du participant (kills/deaths/assists, items,
    /// runes...), serialisees en JSON pour rester agnostiques du schema
    /// exact tout en restant interrogeables via `json_extract` si besoin.
    pub stats_json: String,
}

/// Insere un match et les statistiques d'un participant (typiquement celui
/// dont on synchronise l'historique). Idempotent : rejoue sans erreur si le
/// match est deja connu.
pub fn upsert_match_participant(
    conn: &Connection,
    record: &MatchParticipantRecord,
) -> rusqlite::Result<()> {
    let bans_json =
        serde_json::to_string(&record.banned_champion_ids).unwrap_or_else(|_| "[]".to_string());

    conn.execute(
        "INSERT INTO matches (match_id, queue_id, patch, played_at, duration_seconds, bans_json)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6)
         ON CONFLICT(match_id) DO NOTHING",
        params![
            record.match_id,
            record.queue_id,
            record.patch,
            record.played_at,
            record.duration_seconds,
            bans_json,
        ],
    )?;

    let already_recorded: bool = conn.query_row(
        "SELECT EXISTS(SELECT 1 FROM match_participants WHERE match_id = ?1 AND puuid = ?2)",
        params![record.match_id, record.puuid],
        |row| row.get(0),
    )?;

    if already_recorded {
        return Ok(());
    }

    conn.execute(
        "INSERT INTO match_participants
            (match_id, puuid, champion, champion_id, team_position, win, stats_json)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
        params![
            record.match_id,
            record.puuid,
            record.champion,
            record.champion_id,
            record.team_position,
            record.win as i64,
            record.stats_json,
        ],
    )?;

    Ok(())
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MatchHistoryEntry {
    pub match_id: String,
    pub queue_id: i64,
    pub patch: String,
    pub played_at: String,
    pub duration_seconds: i64,
    pub champion: String,
    pub team_position: String,
    pub win: bool,
    pub stats_json: String,
}

/// Historique le plus recent d'un joueur, tel que deja synchronise en base
/// locale (voir `commands::history::sync_match_history`).
pub fn list_for_puuid(
    conn: &Connection,
    puuid: &str,
    limit: u32,
) -> rusqlite::Result<Vec<MatchHistoryEntry>> {
    let mut statement = conn.prepare(
        "SELECT m.match_id, m.queue_id, m.patch, m.played_at, m.duration_seconds,
                mp.champion, mp.team_position, mp.win, mp.stats_json
         FROM match_participants mp
         JOIN matches m ON m.match_id = mp.match_id
         WHERE mp.puuid = ?1
         ORDER BY m.played_at DESC
         LIMIT ?2",
    )?;

    let rows = statement.query_map(params![puuid, limit], |row| {
        Ok(MatchHistoryEntry {
            match_id: row.get(0)?,
            queue_id: row.get(1)?,
            patch: row.get(2)?,
            played_at: row.get(3)?,
            duration_seconds: row.get(4)?,
            champion: row.get(5)?,
            team_position: row.get(6)?,
            win: row.get::<_, i64>(7)? != 0,
            stats_json: row.get(8)?,
        })
    })?;

    rows.collect()
}

/// Identifiants de matchs deja presents en base pour ce joueur (evite de
/// re-telecharger un match deja synchronise).
pub fn known_match_ids(conn: &Connection, puuid: &str) -> rusqlite::Result<Vec<String>> {
    let mut statement = conn.prepare("SELECT match_id FROM match_participants WHERE puuid = ?1")?;
    let rows = statement.query_map(params![puuid], |row| row.get(0))?;
    rows.collect()
}

/// Nombre de parties synchronisees localement jouees depuis une date donnee
/// (utilise par le suivi d'objectifs "nombre de parties").
pub fn count_since(conn: &Connection, puuid: &str, since: &str) -> rusqlite::Result<i64> {
    conn.query_row(
        "SELECT COUNT(*)
         FROM match_participants mp
         JOIN matches m ON m.match_id = mp.match_id
         WHERE mp.puuid = ?1 AND m.played_at >= ?2",
        params![puuid, since],
        |row| row.get(0),
    )
}

/// Detail complet d'un match synchronise pour un joueur (utilise par le
/// coaching post-partie, qui a besoin du champion_id/role/patch/duree en
/// plus des statistiques brutes).
pub struct MatchParticipantDetail {
    pub champion_id: i64,
    pub team_position: String,
    pub patch: String,
    pub duration_seconds: i64,
    pub win: bool,
    pub stats_json: String,
}

pub fn get_participant(
    conn: &Connection,
    match_id: &str,
    puuid: &str,
) -> rusqlite::Result<Option<MatchParticipantDetail>> {
    conn.query_row(
        "SELECT mp.champion_id, mp.team_position, m.patch, m.duration_seconds, mp.win, mp.stats_json
         FROM match_participants mp
         JOIN matches m ON m.match_id = mp.match_id
         WHERE mp.match_id = ?1 AND mp.puuid = ?2",
        params![match_id, puuid],
        |row| {
            Ok(MatchParticipantDetail {
                champion_id: row.get(0)?,
                team_position: row.get(1)?,
                patch: row.get(2)?,
                duration_seconds: row.get(3)?,
                win: row.get::<_, i64>(4)? != 0,
                stats_json: row.get(5)?,
            })
        },
    )
    .optional()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::infrastructure::db::migrations;

    fn setup() -> Connection {
        let mut conn = Connection::open_in_memory().unwrap();
        migrations::run(&mut conn).unwrap();
        conn
    }

    fn sample(match_id: &str, puuid: &str) -> MatchParticipantRecord {
        MatchParticipantRecord {
            match_id: match_id.to_string(),
            queue_id: 420,
            patch: "14.1".to_string(),
            played_at: "2026-01-01T00:00:00Z".to_string(),
            duration_seconds: 1800,
            banned_champion_ids: vec![1, 2, 3],
            puuid: puuid.to_string(),
            champion: "Ahri".to_string(),
            champion_id: 103,
            team_position: "MIDDLE".to_string(),
            win: true,
            stats_json: "{}".to_string(),
        }
    }

    #[test]
    fn inserts_and_lists_match_history() {
        let conn = setup();
        upsert_match_participant(&conn, &sample("NA1_1", "puuid-1")).unwrap();
        upsert_match_participant(&conn, &sample("NA1_2", "puuid-1")).unwrap();

        let history = list_for_puuid(&conn, "puuid-1", 10).unwrap();
        assert_eq!(history.len(), 2);
    }

    #[test]
    fn is_idempotent_for_the_same_match_and_participant() {
        let conn = setup();
        upsert_match_participant(&conn, &sample("NA1_1", "puuid-1")).unwrap();
        upsert_match_participant(&conn, &sample("NA1_1", "puuid-1")).unwrap();

        let history = list_for_puuid(&conn, "puuid-1", 10).unwrap();
        assert_eq!(history.len(), 1);
    }

    #[test]
    fn known_match_ids_reflects_inserted_matches() {
        let conn = setup();
        upsert_match_participant(&conn, &sample("NA1_1", "puuid-1")).unwrap();

        let ids = known_match_ids(&conn, "puuid-1").unwrap();
        assert_eq!(ids, vec!["NA1_1".to_string()]);
    }

    #[test]
    fn get_participant_returns_match_detail() {
        let conn = setup();
        upsert_match_participant(&conn, &sample("NA1_1", "puuid-1")).unwrap();

        let detail = get_participant(&conn, "NA1_1", "puuid-1").unwrap().unwrap();
        assert_eq!(detail.champion_id, 103);
        assert_eq!(detail.team_position, "MIDDLE");
        assert!(detail.win);
    }

    #[test]
    fn get_participant_returns_none_for_unknown_match() {
        let conn = setup();
        assert!(get_participant(&conn, "NA1_UNKNOWN", "puuid-1")
            .unwrap()
            .is_none());
    }

    #[test]
    fn stores_bans_and_champion_id_for_stats_engine() {
        let conn = setup();
        upsert_match_participant(&conn, &sample("NA1_1", "puuid-1")).unwrap();

        let bans_json: String = conn
            .query_row(
                "SELECT bans_json FROM matches WHERE match_id = 'NA1_1'",
                [],
                |row| row.get(0),
            )
            .unwrap();
        assert_eq!(bans_json, "[1,2,3]");

        let champion_id: i64 = conn
            .query_row(
                "SELECT champion_id FROM match_participants WHERE match_id = 'NA1_1'",
                [],
                |row| row.get(0),
            )
            .unwrap();
        assert_eq!(champion_id, 103);
    }
}
