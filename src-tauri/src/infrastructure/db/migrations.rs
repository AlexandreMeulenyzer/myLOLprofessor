use rusqlite::Connection;

/// Migrations appliquees dans l'ordre, une seule fois chacune (suivies dans
/// `schema_migrations`). Volontairement simple (pas de crate de migration
/// dediee) : le schema de Wardstone reste petit et local, une table de
/// versions suffit et evite une dependance supplementaire.
const MIGRATIONS: &[(&str, &str)] = &[
    (
        "0001_accounts",
        r#"
        CREATE TABLE accounts (
            puuid       TEXT PRIMARY KEY,
            game_name   TEXT NOT NULL,
            tag_line    TEXT NOT NULL,
            platform    TEXT NOT NULL,
            is_primary  INTEGER NOT NULL DEFAULT 0,
            linked_at   TEXT NOT NULL
        );
        "#,
    ),
    (
        "0002_league_snapshots",
        r#"
        CREATE TABLE league_snapshots (
            id           INTEGER PRIMARY KEY AUTOINCREMENT,
            puuid        TEXT NOT NULL REFERENCES accounts(puuid) ON DELETE CASCADE,
            queue_type   TEXT NOT NULL,
            tier         TEXT NOT NULL,
            rank         TEXT NOT NULL,
            league_points INTEGER NOT NULL,
            wins         INTEGER NOT NULL,
            losses       INTEGER NOT NULL,
            captured_at  TEXT NOT NULL
        );
        CREATE INDEX idx_league_snapshots_puuid ON league_snapshots(puuid, captured_at);
        "#,
    ),
    (
        "0003_matches",
        r#"
        CREATE TABLE matches (
            match_id         TEXT PRIMARY KEY,
            queue_id         INTEGER NOT NULL,
            patch            TEXT NOT NULL,
            played_at        TEXT NOT NULL,
            duration_seconds INTEGER NOT NULL
        );

        CREATE TABLE match_participants (
            id                    INTEGER PRIMARY KEY AUTOINCREMENT,
            match_id              TEXT NOT NULL REFERENCES matches(match_id) ON DELETE CASCADE,
            puuid                 TEXT NOT NULL,
            champion              TEXT NOT NULL,
            team_position         TEXT NOT NULL,
            win                   INTEGER NOT NULL,
            stats_json            TEXT NOT NULL
        );
        CREATE INDEX idx_match_participants_puuid ON match_participants(puuid);
        CREATE INDEX idx_match_participants_match ON match_participants(match_id);
        "#,
    ),
    (
        "0004_champion_stats_cache",
        r#"
        CREATE TABLE champion_stats_cache (
            champion       TEXT NOT NULL,
            role           TEXT NOT NULL,
            elo_bucket     TEXT NOT NULL,
            patch          TEXT NOT NULL,
            aggregate_json TEXT NOT NULL,
            computed_at    TEXT NOT NULL,
            PRIMARY KEY (champion, role, elo_bucket, patch)
        );
        "#,
    ),
    (
        "0005_objectives",
        r#"
        CREATE TABLE objectives (
            id          INTEGER PRIMARY KEY AUTOINCREMENT,
            puuid       TEXT NOT NULL REFERENCES accounts(puuid) ON DELETE CASCADE,
            kind        TEXT NOT NULL,
            target_json TEXT NOT NULL,
            created_at  TEXT NOT NULL,
            achieved_at TEXT
        );
        "#,
    ),
    (
        "0006_settings",
        r#"
        CREATE TABLE settings (
            key   TEXT PRIMARY KEY,
            value TEXT NOT NULL
        );
        "#,
    ),
];

pub fn run(conn: &mut Connection) -> rusqlite::Result<()> {
    conn.execute_batch(
        "CREATE TABLE IF NOT EXISTS schema_migrations (name TEXT PRIMARY KEY, applied_at TEXT NOT NULL);",
    )?;

    for (name, sql) in MIGRATIONS {
        let already_applied: bool = conn.query_row(
            "SELECT EXISTS(SELECT 1 FROM schema_migrations WHERE name = ?1)",
            [name],
            |row| row.get(0),
        )?;

        if already_applied {
            continue;
        }

        let tx = conn.transaction()?;
        tx.execute_batch(sql)?;
        tx.execute(
            "INSERT INTO schema_migrations (name, applied_at) VALUES (?1, datetime('now'))",
            [name],
        )?;
        tx.commit()?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn applies_all_migrations_on_a_fresh_database() {
        let mut conn = Connection::open_in_memory().unwrap();
        run(&mut conn).expect("migrations should apply cleanly");

        let table_count: i64 = conn
            .query_row(
                "SELECT COUNT(*) FROM sqlite_master WHERE type = 'table' AND name = 'accounts'",
                [],
                |row| row.get(0),
            )
            .unwrap();
        assert_eq!(table_count, 1);
    }

    #[test]
    fn is_idempotent() {
        let mut conn = Connection::open_in_memory().unwrap();
        run(&mut conn).unwrap();
        run(&mut conn).expect("re-running migrations should be a no-op, not an error");
    }
}
