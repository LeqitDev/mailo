use std::{error::Error, fs, path::PathBuf};

use rusqlite::{params, Connection};
use serde::{Deserialize, Serialize};

pub fn get_database(mut path: PathBuf) -> Result<Connection, Box<dyn Error>> {
    path.push("mail.sqlite3");
    fs::create_dir_all(path.parent().unwrap())?;
    let conn = Connection::open(path)?;
    conn.execute(
        "
        CREATE TABLE IF NOT EXISTS accounts (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            email TEXT NOT NULL,
            username TEXT NOT NULL,
            password TEXT NOT NULL,
            imap_server TEXT NOT NULL,
            imap_port INTEGER NOT NULL
        )
        ",
        (),
    )?;
    conn.execute(
        "
        CREATE TABLE IF NOT EXISTS emails (
            id INTEGER PRIMARY KEY,
            account_id INTEGER NOT NULL,
            subject TEXT NOT NULL,
            sender TEXT NOT NULL,
            date TEXT NOT NULL,
            body TEXT NOT NULL,
            FOREIGN KEY (account_id) REFERENCES accounts (id)
        )
        ",
        (),
    )?;
    Ok(conn)
}

pub trait AccountTable {
    fn get_accounts(&self) -> Result<Vec<Account>, rusqlite::Error>;
    fn get_account(&self, id: i64) -> Result<Account, rusqlite::Error>;
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
        Ok(account[0].clone())
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Account {
    pub id: i64,
    pub email: String,
    pub username: String,
    pub password: String,
    pub imap_server: String,
    pub imap_port: i64,
}

impl Account {
    pub fn new(
        id: i64,
        email: String,
        username: String,
        password: String,
        imap_server: String,
        imap_port: i64,
    ) -> Self {
        Self {
            id,
            email,
            username,
            password,
            imap_server,
            imap_port,
        }
    }

    pub fn push(&self, conn: &Connection) -> Result<(), rusqlite::Error> {
        conn.execute(
            "
            INSERT INTO accounts (email, username, password, imap_server, imap_port)
            VALUES (?1, ?2, ?3, ?4, ?5)
            ",
            params![
                &self.email,
                &self.username,
                &self.password,
                &self.imap_server,
                &self.imap_port,
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
            password: row.get(3)?,
            imap_server: row.get(4)?,
            imap_port: row.get(5)?,
        })
    }
}

pub struct Email {
    pub id: i64,
    pub account_id: i64,
    pub subject: String,
    pub sender: String,
    pub date: String,
    pub body: String,
}

impl Email {
    pub fn new(
        id: i64,
        account_id: i64,
        subject: String,
        sender: String,
        date: String,
        body: String,
    ) -> Self {
        Self {
            id,
            account_id,
            subject,
            sender,
            date,
            body,
        }
    }

    pub fn push(&self, conn: &Connection) -> Result<(), rusqlite::Error> {
        conn.execute(
            "
            INSERT INTO emails (id, account_id, subject, sender, date, body)
            VALUES (?1, ?2, ?3, ?4, ?5, ?6)
            ",
            params![
                &self.id,
                &self.account_id,
                &self.subject,
                &self.sender,
                &self.date,
                &self.body,
            ],
        )?;
        Ok(())
    }
}