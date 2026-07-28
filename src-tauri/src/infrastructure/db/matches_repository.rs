use rusqlite::{params, Connection};
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MatchParticipantRecord {
    pub match_id: String,
    pub queue_id: i64,
    pub patch: String,
    pub played_at: String,
    pub duration_seconds: i64,
    pub puuid: String,
    pub champion: String,
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
    conn.execute(
        "INSERT INTO matches (match_id, queue_id, patch, played_at, duration_seconds)
         VALUES (?1, ?2, ?3, ?4, ?5)
         ON CONFLICT(match_id) DO NOTHING",
        params![
            record.match_id,
            record.queue_id,
            record.patch,
            record.played_at,
            record.duration_seconds,
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
            (match_id, puuid, champion, team_position, win, stats_json)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
        params![
            record.match_id,
            record.puuid,
            record.champion,
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
            puuid: puuid.to_string(),
            champion: "Ahri".to_string(),
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
}
