use rusqlite::{params, Connection};
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct AccountRecord {
    pub puuid: String,
    pub game_name: String,
    pub tag_line: String,
    pub platform: String,
    pub is_primary: bool,
    pub linked_at: String,
}

/// Ajoute un compte Riot lie, ou met a jour ses informations si le puuid
/// existe deja (ex: changement de Riot ID).
pub fn upsert(conn: &Connection, account: &AccountRecord) -> rusqlite::Result<()> {
    conn.execute(
        "INSERT INTO accounts (puuid, game_name, tag_line, platform, is_primary, linked_at)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6)
         ON CONFLICT(puuid) DO UPDATE SET
            game_name = excluded.game_name,
            tag_line = excluded.tag_line,
            platform = excluded.platform",
        params![
            account.puuid,
            account.game_name,
            account.tag_line,
            account.platform,
            account.is_primary as i64,
            account.linked_at,
        ],
    )?;
    Ok(())
}

pub fn list(conn: &Connection) -> rusqlite::Result<Vec<AccountRecord>> {
    let mut statement = conn.prepare(
        "SELECT puuid, game_name, tag_line, platform, is_primary, linked_at
         FROM accounts ORDER BY linked_at ASC",
    )?;

    let rows = statement.query_map([], |row| {
        Ok(AccountRecord {
            puuid: row.get(0)?,
            game_name: row.get(1)?,
            tag_line: row.get(2)?,
            platform: row.get(3)?,
            is_primary: row.get::<_, i64>(4)? != 0,
            linked_at: row.get(5)?,
        })
    })?;

    rows.collect()
}

pub fn delete(conn: &Connection, puuid: &str) -> rusqlite::Result<()> {
    conn.execute("DELETE FROM accounts WHERE puuid = ?1", params![puuid])?;
    Ok(())
}

/// Definit `puuid` comme compte principal (celui affiche par defaut sur le
/// dashboard) et retire ce statut a tous les autres.
pub fn set_primary(conn: &Connection, puuid: &str) -> rusqlite::Result<()> {
    conn.execute("UPDATE accounts SET is_primary = 0", [])?;
    conn.execute(
        "UPDATE accounts SET is_primary = 1 WHERE puuid = ?1",
        params![puuid],
    )?;
    Ok(())
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

    fn sample(puuid: &str, primary: bool) -> AccountRecord {
        AccountRecord {
            puuid: puuid.to_string(),
            game_name: "Faker".to_string(),
            tag_line: "KR1".to_string(),
            platform: "kr".to_string(),
            is_primary: primary,
            linked_at: "2026-01-01T00:00:00Z".to_string(),
        }
    }

    #[test]
    fn inserts_and_lists_accounts() {
        let conn = setup();
        upsert(&conn, &sample("puuid-1", true)).unwrap();

        let accounts = list(&conn).unwrap();
        assert_eq!(accounts.len(), 1);
        assert_eq!(accounts[0].game_name, "Faker");
        assert!(accounts[0].is_primary);
    }

    #[test]
    fn upsert_updates_existing_account_by_puuid() {
        let conn = setup();
        upsert(&conn, &sample("puuid-1", true)).unwrap();

        let mut renamed = sample("puuid-1", true);
        renamed.game_name = "T1 Faker".to_string();
        upsert(&conn, &renamed).unwrap();

        let accounts = list(&conn).unwrap();
        assert_eq!(accounts.len(), 1);
        assert_eq!(accounts[0].game_name, "T1 Faker");
    }

    #[test]
    fn set_primary_ensures_a_single_primary_account() {
        let conn = setup();
        upsert(&conn, &sample("puuid-1", true)).unwrap();
        upsert(&conn, &sample("puuid-2", false)).unwrap();

        set_primary(&conn, "puuid-2").unwrap();

        let accounts = list(&conn).unwrap();
        let primary_count = accounts.iter().filter(|a| a.is_primary).count();
        assert_eq!(primary_count, 1);
        assert!(
            accounts
                .iter()
                .find(|a| a.puuid == "puuid-2")
                .unwrap()
                .is_primary
        );
    }

    #[test]
    fn delete_removes_the_account() {
        let conn = setup();
        upsert(&conn, &sample("puuid-1", true)).unwrap();
        delete(&conn, "puuid-1").unwrap();
        assert!(list(&conn).unwrap().is_empty());
    }
}
