// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod database;

use database::{get_database, Account, AccountTable};
use rusqlite::Connection;
use std::{
    path::PathBuf,
    sync::{Arc, Mutex},
};
use tauri::Manager;

struct AppState(Arc<Mutex<Sharable>>);

#[derive(Default)]
struct Sharable {
    sql: Option<Connection>,
}

impl Sharable {
    fn new(path: PathBuf) -> Self {
        match get_database(path) {
            Ok(conn) => Self { sql: Some(conn) },
            Err(e) => {
                println!("Failed to get database connection: {:#?}", e);
                Self { sql: None }
            }
        }
    }
}

#[tauri::command]
fn get_accounts(state: tauri::State<AppState>) -> Result<Vec<database::Account>, String> {
    if let Ok(state) = state.0.lock() {
        if let Some(conn) = &state.sql {
            conn.get_accounts().map_err(|e| e.to_string())
        } else {
            Err("Failed to get database connection".to_string())
        }
    } else {
        Err("Failed to lock state".to_string())
    }
}

#[tauri::command]
fn add_account(
    state: tauri::State<AppState>,
    email: String,
    username: String,
    password: String,
    imap_host: String,
    imap_port: i64,
) -> Result<(), String> {
    println!("Adding account: {:#?}", email);
    if let Ok(state) = state.0.lock() {
        if let Some(conn) = state.sql.as_ref() {
            println!("Added account: {:#?}", email);
            Account::new(-1, email, username, password, imap_host, imap_port)
                .push(conn)
                .map_err(|e| e.to_string())
        } else {
            Err("Failed to get database connection".to_string())
        }
    } else {
        Err("Failed to lock state".to_string())
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_accounts, add_account])
        .setup(|app| {
            let handle = app.app_handle();
            app.manage(AppState(Arc::new(Mutex::new(Sharable::new(
                handle.path_resolver().app_data_dir().unwrap(),
            )))));
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
