use rusqlite::{params, Connection};
use serde::Serialize;

pub struct LeagueSnapshotInput<'a> {
    pub puuid: &'a str,
    pub queue_type: &'a str,
    pub tier: &'a str,
    pub rank: &'a str,
    pub league_points: i64,
    pub wins: i64,
    pub losses: i64,
    pub captured_at: &'a str,
}

/// Enregistre un instantane de rang. Appele a chaque consultation du profil
/// (voir `commands::profile::get_profile`) : la frequence naturelle des
/// visites de l'utilisateur suffit a tracer une courbe de progression sans
/// tache de fond dediee.
pub fn record_snapshot(conn: &Connection, input: LeagueSnapshotInput<'_>) -> rusqlite::Result<()> {
    conn.execute(
        "INSERT INTO league_snapshots (puuid, queue_type, tier, rank, league_points, wins, losses, captured_at)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
        params![
            input.puuid,
            input.queue_type,
            input.tier,
            input.rank,
            input.league_points,
            input.wins,
            input.losses,
            input.captured_at,
        ],
    )?;
    Ok(())
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LeagueSnapshotRecord {
    pub tier: String,
    pub rank: String,
    pub league_points: i64,
    pub wins: i64,
    pub losses: i64,
    pub captured_at: String,
}

pub fn list_for_puuid(
    conn: &Connection,
    puuid: &str,
    queue_type: &str,
    limit: u32,
) -> rusqlite::Result<Vec<LeagueSnapshotRecord>> {
    let mut statement = conn.prepare(
        "SELECT tier, rank, league_points, wins, losses, captured_at
         FROM league_snapshots
         WHERE puuid = ?1 AND queue_type = ?2
         ORDER BY captured_at ASC
         LIMIT ?3",
    )?;

    let rows = statement.query_map(params![puuid, queue_type, limit], |row| {
        Ok(LeagueSnapshotRecord {
            tier: row.get(0)?,
            rank: row.get(1)?,
            league_points: row.get(2)?,
            wins: row.get(3)?,
            losses: row.get(4)?,
            captured_at: row.get(5)?,
        })
    })?;

    rows.collect()
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

    fn input<'a>(
        queue_type: &'a str,
        tier: &'a str,
        rank: &'a str,
        league_points: i64,
        wins: i64,
        losses: i64,
        captured_at: &'a str,
    ) -> LeagueSnapshotInput<'a> {
        LeagueSnapshotInput {
            puuid: "puuid-1",
            queue_type,
            tier,
            rank,
            league_points,
            wins,
            losses,
            captured_at,
        }
    }

    #[test]
    fn records_and_lists_snapshots_in_chronological_order() {
        let conn = setup();
        record_snapshot(
            &conn,
            input(
                "RANKED_SOLO_5x5",
                "GOLD",
                "IV",
                20,
                10,
                8,
                "2026-01-01T00:00:00Z",
            ),
        )
        .unwrap();
        record_snapshot(
            &conn,
            input(
                "RANKED_SOLO_5x5",
                "GOLD",
                "III",
                5,
                12,
                8,
                "2026-01-05T00:00:00Z",
            ),
        )
        .unwrap();

        let snapshots = list_for_puuid(&conn, "puuid-1", "RANKED_SOLO_5x5", 10).unwrap();
        assert_eq!(snapshots.len(), 2);
        assert_eq!(snapshots[0].captured_at, "2026-01-01T00:00:00Z");
        assert_eq!(snapshots[1].rank, "III");
    }

    #[test]
    fn filters_by_queue_type() {
        let conn = setup();
        record_snapshot(
            &conn,
            input(
                "RANKED_SOLO_5x5",
                "GOLD",
                "IV",
                20,
                10,
                8,
                "2026-01-01T00:00:00Z",
            ),
        )
        .unwrap();
        record_snapshot(
            &conn,
            input(
                "RANKED_FLEX_SR",
                "SILVER",
                "I",
                40,
                5,
                3,
                "2026-01-01T00:00:00Z",
            ),
        )
        .unwrap();

        let solo = list_for_puuid(&conn, "puuid-1", "RANKED_SOLO_5x5", 10).unwrap();
        assert_eq!(solo.len(), 1);
        assert_eq!(solo[0].tier, "GOLD");
    }
}
