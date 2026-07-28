pub mod accounts_repository;
pub mod league_snapshots_repository;
pub mod matches_repository;
pub mod migrations;

use std::path::Path;
use std::sync::Mutex;

use rusqlite::Connection;

/// Point d'acces unique a la base SQLite locale de Wardstone. `rusqlite`
/// est synchrone ; pour une application desktop mono-utilisateur avec une
/// base locale rapide, un simple mutex autour de la connexion est suffisant
/// et evite la complexite d'un pool de connexions.
pub struct Database {
    conn: Mutex<Connection>,
}

impl Database {
    pub fn open(path: &Path) -> rusqlite::Result<Self> {
        let mut conn = Connection::open(path)?;
        conn.execute_batch("PRAGMA foreign_keys = ON; PRAGMA journal_mode = WAL;")?;
        migrations::run(&mut conn)?;
        Ok(Self {
            conn: Mutex::new(conn),
        })
    }

    #[cfg(test)]
    pub fn open_in_memory() -> rusqlite::Result<Self> {
        let mut conn = Connection::open_in_memory()?;
        migrations::run(&mut conn)?;
        Ok(Self {
            conn: Mutex::new(conn),
        })
    }

    pub fn lock(&self) -> std::sync::MutexGuard<'_, Connection> {
        self.conn
            .lock()
            .expect("mutex de connexion SQLite empoisonne")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::infrastructure::db::accounts_repository::{self, AccountRecord};

    #[test]
    fn open_in_memory_runs_migrations_and_allows_repository_access() {
        let db = Database::open_in_memory().expect("in-memory database should open");

        accounts_repository::upsert(
            &db.lock(),
            &AccountRecord {
                puuid: "puuid-1".to_string(),
                game_name: "Faker".to_string(),
                tag_line: "KR1".to_string(),
                platform: "kr".to_string(),
                is_primary: true,
                linked_at: "2026-01-01T00:00:00Z".to_string(),
            },
        )
        .expect("upsert should succeed on a migrated database");

        let accounts = accounts_repository::list(&db.lock()).unwrap();
        assert_eq!(accounts.len(), 1);
    }
}
