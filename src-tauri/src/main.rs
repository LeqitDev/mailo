// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod database;
mod imap;

use database::{get_database, Account, AccountTable, Email, EmailTable};
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
    events: Vec<FrontendEvent>,
    backend_closed: bool,
}

impl Shareble {
    fn new(path: PathBuf) -> Self {
        match get_database(path) {
            Ok(conn) => Self {
                sql: Some(conn),
                logout: false,
                frontend_ready: false,
                events: Vec::new(),
                backend_closed: false,
            },
            Err(e) => {
                println!("Failed to get database connection: {:#?}", e);
                Self {
                    sql: None,
                    logout: false,
                    frontend_ready: false,
                    events: Vec::new(),
                    backend_closed: false,
                }
            }
        }
    }

    fn push_log<T: ToString>(&mut self, message: T, log_type: LoggerType) {
        self.events.push(FrontendEvent::Log(LoggerPayload {
            message: message.to_string(),
            log_type,
        }));
    }

    fn push_action<T: ToString>(&mut self, action: T, payload: T) {
        self.events.push(FrontendEvent::Action(ActionPayload {
            action: action.to_string(),
            payload: payload.to_string(),
        }));
    }
}

trait EventDispatcher {
    fn log<T: ToString>(&mut self, message: T, log_type: LoggerType);
    fn log_error<T: ToString>(&mut self, message: T) {
        self.log(message, LoggerType::Error);
    }
    fn log_warning<T: ToString>(&mut self, message: T) {
        self.log(message, LoggerType::Warning);
    }
    fn log_info<T: ToString>(&mut self, message: T) {
        self.log(message, LoggerType::Info);
    }
    fn action<T: ToString>(&mut self, action: T, payload: T);
}

impl EventDispatcher for Arc<Mutex<Shareble>> {
    fn log<T: ToString>(&mut self, message: T, log_type: LoggerType) {
        self.lock().unwrap().push_log(message, log_type);
    }
    fn action<T: ToString>(&mut self, action: T, payload: T) {
        self.lock().unwrap().push_action(action, payload);
    }
}

#[derive(Clone, Debug)]
enum FrontendEvent {
    Log(LoggerPayload),
    Action(ActionPayload),
}

#[derive(Clone, Serialize, Debug)]
struct ActionPayload {
    action: String,
    payload: String,
}

#[derive(Clone, Serialize, Debug)]
enum LoggerType {
    Info,
    Warning,
    Error,
}

#[derive(Clone, Serialize, Debug)]
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
    mut state: tauri::State<AppState>,
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
fn get_emails(state: tauri::State<AppState>) -> Result<Vec<Email>, String> {
    if let Ok(state) = state.0.lock() {
        if let Some(conn) = &state.sql {
            let mut emails = vec![];
            for account in conn.get_accounts().map_err(|e| e.to_string())? {
                emails.append(
                    conn.get_emails(account.id)
                        .map_err(|e| e.to_string())?
                        .as_mut(),
                );
            }
            Ok(emails)
        } else {
            Err("Failed to get database connection".to_string())
        }
    } else {
        Err("Failed to lock state".to_string())
    }
}

#[tauri::command]
fn get_top_emails(state: tauri::State<AppState>) -> Result<Vec<Email>, String> {
    if let Ok(state) = state.0.lock() {
        if let Some(conn) = &state.sql {
            let emails = conn.get_last_hundred_emails(1).map_err(|e| e.to_string())?;
            Ok(emails)
        } else {
            Err("Failed to get database connection".to_string())
        }
    } else {
        Err("Failed to lock state".to_string())
    }
}

#[tauri::command]
fn get_email(state: tauri::State<AppState>, id: i64) -> Result<Email, String> {
    if let Ok(state) = state.0.lock() {
        if let Some(conn) = &state.sql {
            let email = conn.get_email(id).map_err(|e| e.to_string())?;
            Ok(email)
        } else {
            Err("Failed to get database connection".to_string())
        }
    } else {
        Err("Failed to lock state".to_string())
    }
}

#[tauri::command]
async fn logout(state: tauri::State<'_, AppState>, app: AppHandle) -> Result<(), String> {
    if let Ok(mut state) = state.0.lock() {
        state.logout = true;
    }
    loop {
        if state.0.lock().unwrap().backend_closed {
            break;
        }
        tokio::time::sleep(tokio::time::Duration::from_millis(1000)).await;
    }
    app.exit(0);
    Ok(())
}

#[tauri::command]
fn ready(state: tauri::State<AppState>, app: AppHandle) {
    if let Ok(mut state) = state.0.lock() {
        state.frontend_ready = true;
    }
}

#[tauri::command]
fn fetch_logs(state: tauri::State<AppState>) -> Result<Vec<LoggerPayload>, String> {
    if let Ok(mut state) = state.0.lock() {
        // Filter the log events and return them
        let logs = state
            .events
            .iter()
            .filter_map(|event| match event {
                FrontendEvent::Log(log) => Some(log.clone()),
                _ => None,
            })
            .collect();
        state.events.retain(|event| match event {
            FrontendEvent::Log(_) => false,
            _ => true,
        });
        Ok(logs)
    } else {
        Err("Failed to lock state".to_string())
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            get_accounts,
            add_account,
            logout,
            ready,
            get_top_emails,
            get_emails,
            get_email,
            fetch_logs
        ])
        .setup(|app| {
            let handle = app.handle();
            app.manage(AppState(Arc::new(Mutex::new(Shareble::new(
                handle.path_resolver().app_data_dir().unwrap(),
            )))));

            let app_state: Arc<Mutex<Shareble>> = Arc::clone(&app.state::<AppState>().inner().0);
            let second_handle = handle.clone();

            tauri::async_runtime::spawn(async move {
                loop {
                    if app_state.lock().unwrap().logout {
                        break;
                    }
                    if app_state.lock().unwrap().frontend_ready {
                        let mut locked_app_state = app_state.lock().unwrap();
                        let events = locked_app_state.events.clone();
                        locked_app_state.events.clear();
                        drop(locked_app_state);
                        if !events.is_empty() {
                            println!("Sending events to frontend: {:#?}", events);
                            // weird thing the third event does not fire
                            for event in events {
                                match event {
                                    FrontendEvent::Log(log) => second_handle
                                        .get_window("main")
                                        .unwrap()
                                        .emit("log", log)
                                        .unwrap(),
                                    FrontendEvent::Action(action) => second_handle
                                        .get_window("main")
                                        .unwrap()
                                        .emit("action", action)
                                        .unwrap(),
                                }
                            }
                        }
                    }
                    tokio::time::sleep(tokio::time::Duration::from_millis(1000)).await;
                }
            });

            for account in app
                .state::<AppState>()
                .inner()
                .0
                .lock()
                .unwrap()
                .sql
                .as_ref()
                .unwrap()
                .get_accounts()
                .unwrap()
            {
                let app_state: Arc<Mutex<Shareble>> =
                    Arc::clone(&app.state::<AppState>().inner().0);

                tauri::async_runtime::spawn(async move {
                    imap(app_state, account).await;
                });
            }
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
