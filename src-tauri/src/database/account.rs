use std::str::from_utf8;

use rusqlite::{params, Connection};
use base64::prelude::*;
use serde::{Deserialize, Serialize};

pub trait AccountTable {
    fn get_accounts(&self) -> Result<Vec<Account>, rusqlite::Error>;
    fn get_account(&self, id: i64) -> Result<Account, rusqlite::Error>;
    fn delete_account(&self, id: i64) -> Result<(), rusqlite::Error>;
}

impl AccountTable for Connection {
    fn get_accounts(&self) -> Result<Vec<Account>, rusqlite::Error> {
        let mut stmt = self.prepare("SELECT * FROM accounts")?;
        let accounts = stmt
            .query_map(params![], |row| Ok(Account::try_from(row).unwrap()))?
            .collect::<Result<Vec<Account>, _>>()?;
        Ok(accounts)
    }

    fn get_account(&self, id: i64) -> Result<Account, rusqlite::Error> {
        let mut stmt = self.prepare("SELECT * FROM accounts WHERE id = ?1")?;
        let account = stmt
            .query_map(params![id], |row| Ok(Account::try_from(row).unwrap()))?
            .collect::<Result<Vec<Account>, _>>()?;
        if account.is_empty() {
            return Err(rusqlite::Error::QueryReturnedNoRows);
        }
        Ok(account[0].clone())
    }

    fn delete_account(&self, id: i64) -> Result<(), rusqlite::Error> {
        self.execute("DELETE FROM accounts WHERE id = ?1", params![id])?;
        Ok(())
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Account {
    pub id: i64,
    pub email: String,
    pub username: String,
    pub password: String,
    pub imap_host: String,
    pub imap_port: i64,
    pub display_name: Option<String>,
}

impl Account {
    pub fn new(
        id: i64,
        email: String,
        username: String,
        password: String,
        imap_server: String,
        imap_port: i64,
        display_name: Option<String>,
    ) -> Self {
        Self {
            id,
            email,
            username,
            password,
            imap_host: imap_server,
            imap_port,
            display_name,
        }
    }

    pub fn push(&self, conn: &Connection) -> Result<(), rusqlite::Error> {
        conn.execute(
            "
            INSERT OR IGNORE INTO accounts (email, username, password, imap_server, imap_port)
            VALUES (?1, ?2, ?3, ?4, ?5)
            ",
            params![
                &self.email,
                &self.username,
                &self.password,
                &self.imap_host,
                &self.imap_port,
            ],
        )?;
        Ok(())
    }

    pub fn update(&self, conn: &Connection) -> Result<(), rusqlite::Error> {
        conn.execute(
            "
            UPDATE accounts
            SET email = ?1, username = ?2, password = ?3, imap_server = ?4, imap_port = ?5, display_name = ?6
            WHERE id = ?7
            ",
            params![
                &self.email,
                &self.username,
                &self.password,
                &self.imap_host,
                &self.imap_port,
                &self.display_name,
                &self.id,
            ],
        )?;
        Ok(())
    }
}

impl TryFrom<&rusqlite::Row<'_>> for Account {
    type Error = rusqlite::Error;

    fn try_from(row: &rusqlite::Row) -> Result<Self, Self::Error> {
        Ok(Self {
            id: row.get(0)?,
            email: row.get(1)?,
            username: row.get(2)?,
            password: from_utf8(
                BASE64_STANDARD
                    .decode(row.get::<usize, String>(3)?.as_bytes())
                    .unwrap_or_default()
                    .as_slice(),
            )
            .unwrap_or("No password found!")
            .to_string(),
            imap_host: row.get(4)?,
            imap_port: row.get(5)?,
            display_name: row.get(6)?,
        })
    }
}