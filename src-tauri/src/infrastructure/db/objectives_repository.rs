use rusqlite::{params, Connection};
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ObjectiveRecord {
    pub id: i64,
    pub puuid: String,
    /// JSON serialise de `domain::objective::ObjectiveKind`.
    pub kind_json: String,
    pub created_at: String,
    pub achieved_at: Option<String>,
}

/// `kind_tag` est le discriminant lisible (ex: "reachRank") stocke pour
/// permettre un filtrage SQL futur ; `kind_json` est la representation
/// complete de `domain::objective::ObjectiveKind` (source de verite,
/// desserialisee telle quelle par les commandes).
pub fn insert(
    conn: &Connection,
    puuid: &str,
    kind_tag: &str,
    kind_json: &str,
    created_at: &str,
) -> rusqlite::Result<i64> {
    conn.execute(
        "INSERT INTO objectives (puuid, kind, target_json, created_at, achieved_at)
         VALUES (?1, ?2, ?3, ?4, NULL)",
        params![puuid, kind_tag, kind_json, created_at],
    )?;
    Ok(conn.last_insert_rowid())
}

pub fn list_for_puuid(conn: &Connection, puuid: &str) -> rusqlite::Result<Vec<ObjectiveRecord>> {
    let mut statement = conn.prepare(
        "SELECT id, puuid, target_json, created_at, achieved_at
         FROM objectives
         WHERE puuid = ?1
         ORDER BY created_at DESC",
    )?;

    let rows = statement.query_map(params![puuid], |row| {
        Ok(ObjectiveRecord {
            id: row.get(0)?,
            puuid: row.get(1)?,
            kind_json: row.get(2)?,
            created_at: row.get(3)?,
            achieved_at: row.get(4)?,
        })
    })?;

    rows.collect()
}

pub fn delete(conn: &Connection, id: i64, puuid: &str) -> rusqlite::Result<()> {
    conn.execute(
        "DELETE FROM objectives WHERE id = ?1 AND puuid = ?2",
        params![id, puuid],
    )?;
    Ok(())
}

pub fn mark_achieved(conn: &Connection, id: i64, achieved_at: &str) -> rusqlite::Result<()> {
    conn.execute(
        "UPDATE objectives SET achieved_at = ?1 WHERE id = ?2 AND achieved_at IS NULL",
        params![achieved_at, id],
    )?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::infrastructure::db::accounts_repository::{self, AccountRecord};
    use crate::infrastructure::db::migrations;

    fn setup() -> Connection {
        let mut conn = Connection::open_in_memory().unwrap();
        conn.execute_batch("PRAGMA foreign_keys = ON;").unwrap();
        migrations::run(&mut conn).unwrap();
        accounts_repository::upsert(
            &conn,
            &AccountRecord {
                puuid: "puuid-1".to_string(),
                game_name: "Faker".to_string(),
                tag_line: "KR1".to_string(),
                platform: "kr".to_string(),
                is_primary: true,
                linked_at: "2026-01-01T00:00:00Z".to_string(),
            },
        )
        .unwrap();
        conn
    }

    #[test]
    fn inserts_and_lists_objectives() {
        let conn = setup();
        insert(
            &conn,
            "puuid-1",
            "gamesPlayed",
            "{\"type\":\"gamesPlayed\",\"count\":50}",
            "2026-01-01T00:00:00Z",
        )
        .unwrap();

        let objectives = list_for_puuid(&conn, "puuid-1").unwrap();
        assert_eq!(objectives.len(), 1);
        assert!(objectives[0].achieved_at.is_none());
    }

    #[test]
    fn mark_achieved_sets_the_timestamp_once() {
        let conn = setup();
        let id = insert(
            &conn,
            "puuid-1",
            "gamesPlayed",
            "{\"type\":\"gamesPlayed\",\"count\":50}",
            "2026-01-01T00:00:00Z",
        )
        .unwrap();

        mark_achieved(&conn, id, "2026-02-01T00:00:00Z").unwrap();
        let objectives = list_for_puuid(&conn, "puuid-1").unwrap();
        assert_eq!(
            objectives[0].achieved_at.as_deref(),
            Some("2026-02-01T00:00:00Z")
        );

        // Un second appel ne doit pas ecraser la date d'obtention.
        mark_achieved(&conn, id, "2026-03-01T00:00:00Z").unwrap();
        let objectives = list_for_puuid(&conn, "puuid-1").unwrap();
        assert_eq!(
            objectives[0].achieved_at.as_deref(),
            Some("2026-02-01T00:00:00Z")
        );
    }

    #[test]
    fn delete_removes_the_objective() {
        let conn = setup();
        let id = insert(
            &conn,
            "puuid-1",
            "gamesPlayed",
            "{\"type\":\"gamesPlayed\",\"count\":50}",
            "2026-01-01T00:00:00Z",
        )
        .unwrap();
        delete(&conn, id, "puuid-1").unwrap();
        assert!(list_for_puuid(&conn, "puuid-1").unwrap().is_empty());
    }
}
