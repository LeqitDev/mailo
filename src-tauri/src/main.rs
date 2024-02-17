// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod database;
mod imap;

use database::{get_database, Account, AccountTable};
use imap::imap;
use rusqlite::Connection;
use serde::Serialize;
use std::{
    path::PathBuf,
    sync::{Arc, Mutex},
};
use tauri::{AppHandle, Manager, State};
use tokio::net::TcpStream;

struct AppState(Arc<Mutex<Shareble>>);

#[derive(Default)]
struct Shareble {
    sql: Option<Connection>,
    logout: bool,
    frontend_ready: bool,
    saved_logs: Vec<LoggerPayload>,
}

impl Shareble {
    fn new(path: PathBuf) -> Self {
        match get_database(path) {
            Ok(conn) => Self {
                sql: Some(conn),
                logout: false,
                frontend_ready: false,
                saved_logs: Vec::new(),
            },
            Err(e) => {
                println!("Failed to get database connection: {:#?}", e);
                Self {
                    sql: None,
                    logout: false,
                    frontend_ready: false,
                    saved_logs: Vec::new(),
                }
            }
        }
    }
}

#[derive(Clone, Serialize)]
enum LoggerType {
    Info,
    Warning,
    Error,
}

#[derive(Clone, Serialize)]
struct LoggerPayload {
    message: String,
    log_type: LoggerType,
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

#[tauri::command]
fn logout(state: tauri::State<AppState>, app: AppHandle) {
    println!("Logging out!");
    if let Ok(mut state) = state.0.lock() {
        state.logout = true;
    }
    app.exit(0);
}

#[tauri::command]
fn ready(state: tauri::State<AppState>, app: AppHandle) {
    if let Ok(mut state) = state.0.lock() {
        state.frontend_ready = true;
        for log in state.saved_logs.iter() {
            logging_handler(log.message.clone(), log.log_type.clone(), &app);
        }
        state.saved_logs.clear();
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            get_accounts,
            add_account,
            logout,
            ready
        ])
        .setup(|app| {
            let handle = app.handle();
            app.manage(AppState(Arc::new(Mutex::new(Shareble::new(
                handle.path_resolver().app_data_dir().unwrap(),
            )))));
            let app_state: Arc<Mutex<Shareble>> = Arc::clone(&app.state::<AppState>().inner().0);

            tauri::async_runtime::spawn(async move {
                imap(app_state, &handle).await;
            });
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn logging_handler<R: tauri::Runtime, S: ToString>(
    message: S,
    log_type: LoggerType,
    handle: &impl Manager<R>,
) {
    let app_state = handle.state::<AppState>().inner().0.clone();
    if app_state.lock().unwrap().frontend_ready {
        handle
            .get_window("main")
            .unwrap()
            .emit(
                "log",
                LoggerPayload {
                    message: message.to_string(),
                    log_type,
                },
            )
            .unwrap();
    } else {
        app_state.lock().unwrap().saved_logs.push(LoggerPayload {
            message: message.to_string(),
            log_type,
        });
    }
}
