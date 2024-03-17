use std::error::Error;
use std::fs;
use std::path::PathBuf;

use rusqlite::Connection;
use rusqlite_migration::{Migrations, M};

pub mod account;
pub mod email;

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