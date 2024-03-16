use std::{error::Error, fs, path::PathBuf, str::from_utf8};

use async_imap::types::Flag;
use base64::prelude::*;
use rusqlite::{params, Connection};
use serde::{Deserialize, Serialize};
use rusqlite_migration::{Migrations, M};

pub fn get_database(mut path: PathBuf) -> Result<Connection, Box<dyn Error>> {
    path.push("mail.sqlite3");
    fs::create_dir_all(path.parent().unwrap())?;
    let mut conn = Connection::open(path)?;
    let migrations = Migrations::new(vec![
        M::up("
        CREATE TABLE IF NOT EXISTS accounts (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            email TEXT NOT NULL UNIQUE,
            username TEXT NOT NULL,
            password TEXT NOT NULL,
            imap_server TEXT NOT NULL,
            imap_port INTEGER NOT NULL,
            display_name TEXT
        )"),
        M::up("
        CREATE TABLE IF NOT EXISTS emails (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            email_id TEXT NOT NULL UNIQUE,
            account_id INTEGER NOT NULL,
            subject TEXT NOT NULL,
            sender TEXT NOT NULL,
            date TEXT NOT NULL,
            body TEXT NOT NULL,
            flags TEXT NOT NULL,
            FOREIGN KEY (account_id) REFERENCES accounts (id)
        )")
    ]);
    // conn.pragma_update(None, "journal_mode", &"WAL")?;
    migrations.to_latest(&mut conn)?;
    Ok(conn)
}

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

pub trait EmailTable {
    fn get_emails(&self, account_id: i64) -> Result<Vec<Email>, rusqlite::Error>;
    fn get_last_hundred_emails(&self, account_id: i64) -> Result<Vec<Email>, rusqlite::Error>;
    fn get_email(&self, id: i64) -> Result<Email, rusqlite::Error>;
    fn get_email_count(&self, account_id: i64) -> Result<u32, rusqlite::Error>;
}

impl EmailTable for Connection {
    fn get_emails(&self, account_id: i64) -> Result<Vec<Email>, rusqlite::Error> {
        let mut stmt = self.prepare("SELECT * FROM emails WHERE account_id = ?1")?;
        let emails = stmt
            .query_map(params![account_id], |row| Ok(Email::try_from(row).unwrap()))?
            .collect::<Result<Vec<Email>, _>>()?;
        Ok(emails)
    }

    fn get_last_hundred_emails(&self, account_id: i64) -> Result<Vec<Email>, rusqlite::Error> {
        let mut stmt = self
            .prepare("SELECT * FROM emails WHERE account_id = ?1 ORDER BY date DESC LIMIT 100")?;
        let emails = stmt
            .query_map(params![account_id], |row| Ok(Email::try_from(row).unwrap()))?
            .collect::<Result<Vec<Email>, _>>()?;
        Ok(emails)
    }

    fn get_email(&self, id: i64) -> Result<Email, rusqlite::Error> {
        let mut stmt = self.prepare("SELECT * FROM emails WHERE id = ?1 LIMIT 1")?;
        let email = stmt
            .query_map(params![id], |row| Ok(Email::try_from(row).unwrap()))?
            .collect::<Result<Vec<Email>, _>>()?;
        if email.is_empty() {
            return Err(rusqlite::Error::QueryReturnedNoRows);
        }
        Ok(email[0].clone())
    }

    fn get_email_count(&self, account_id: i64) -> Result<u32, rusqlite::Error> {
        let mut stmt = self.prepare("SELECT COUNT(id) FROM emails WHERE account_id = ?1")?;
        let count = stmt
            .query_map(params![account_id], |row| row.get(0))?
            .next()
            .unwrap()?;
        Ok(count)
    }
}

#[derive(Clone, Serialize, Deserialize, Default)]
pub struct EmailFlags {
    pub seen: bool,
    answered: bool,
    flagged: bool,
    deleted: bool,
    draft: bool,
    recent: bool,
    may_create: bool,
    custom: Vec<String>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Email {
    pub id: i64,
    pub email_id: String,
    pub account_id: i64,
    pub subject: String,
    pub sender: String,
    pub date: String,
    pub body: String,
    pub flags: EmailFlags,
}

impl Email {
    pub fn new(
        id: i64,
        email_id: String,
        account_id: i64,
        subject: String,
        sender: String,
        date: String,
        body: String,
        flags: EmailFlags,
    ) -> Self {
        Self {
            id,
            email_id,
            account_id,
            subject,
            sender,
            date,
            body,
            flags,
        }
    }

    pub fn set_flags(&mut self, flags: Vec<Flag<'_>>) {
        for flag in flags {
            match flag {
                Flag::Seen => self.flags.seen = true,
                Flag::Answered => self.flags.answered = true,
                Flag::Flagged => self.flags.flagged = true,
                Flag::Deleted => self.flags.deleted = true,
                Flag::Draft => self.flags.draft = true,
                Flag::Recent => self.flags.recent = true,
                Flag::MayCreate => self.flags.may_create = true,
                Flag::Custom(c) => self.flags.custom.push(c.to_string()),
            }
        }
    }

    pub fn push(&self, conn: &Connection) -> Result<(), rusqlite::Error> {
        conn.execute(
            "
            INSERT OR REPLACE INTO emails (email_id, account_id, subject, sender, date, body, flags)
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)
            ",
            params![
                &self.email_id,
                &self.account_id,
                &self.subject,
                &self.sender,
                &self.date,
                &self.body,
                serde_json::to_string(&self.flags).unwrap(),
            ],
        )?;
        Ok(())
    }
}

impl TryFrom<&rusqlite::Row<'_>> for Email {
    type Error = rusqlite::Error;

    fn try_from(row: &rusqlite::Row) -> Result<Self, Self::Error> {
        Ok(Self {
            id: row.get(0)?,
            email_id: row.get(1)?,
            account_id: row.get(2)?,
            subject: row.get(3)?,
            sender: row.get(4)?,
            date: row.get(5)?,
            body: row.get(6)?,
            flags: serde_json::from_str(row.get::<usize, String>(7)?.as_str()).unwrap(),
        })
    }
}

impl From<mail_parser::Message<'_>> for Email {
    fn from(message: mail_parser::Message) -> Self {
        let subject = message.subject().unwrap_or("No Subject");
        let email_id = message.message_id().unwrap().to_string();
        let sender = {
            let from = message.from().unwrap();
            let name = {
                let name = from.first().unwrap().name.clone();
                if let Some(name) = name {
                    name.to_string()
                } else {
                    "".to_string()
                }
            };
            let address = {
                let address = from.first().unwrap().address.clone();
                if let Some(address) = address {
                    address.to_string()
                } else {
                    "".to_string()
                }
            };
            format!("{} <{}>", name, address)
        };
        let date = {
            let date = message.date().unwrap();
            format!(
                "{}-{}-{} {}:{}:{}",
                date.year, date.month, date.day, date.hour, date.minute, date.second
            )
        };
        let body = message.body_html(0).unwrap().to_string();
        Self {
            id: -1,
            email_id,
            account_id: -1,
            subject: subject.to_string(),
            sender,
            date,
            body,
            flags: EmailFlags::default(),
        }
    }
}
