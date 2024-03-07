// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app_state;
mod database;
mod imap;
mod settings;

use app_state::{AccountAccessor, AppState, FrontendEvent, LoggerPayload, Shareble};
use base64::prelude::*;
use database::{Account, AccountTable, Email, EmailTable};
use imap::imap;
use std::sync::{Arc, Mutex};
use tauri::{AppHandle, Manager};

#[tauri::command]
fn get_account(state: tauri::State<AppState>, id: i64) -> Result<Account, String> {
    if let Ok(state) = state.0.lock() {
        if let Some(conn) = &state.sql {
            match conn.get_account(id) {
                Ok(account) => Ok(account),
                Err(e) => Err(e.to_string()),
            }
        } else {
            Err("Failed to get database connection".to_string())
        }
    } else {
        Err("Failed to lock state".to_string())
    }
}

#[tauri::command]
fn get_accounts(state: tauri::State<AppState>) -> Result<Vec<database::Account>, String> {
    if let Ok(state) = state.0.lock() {
        if let Some(conn) = &state.sql {
            match conn.get_accounts() {
                Ok(accounts) => Ok(accounts),
                Err(e) => Err(e.to_string()),
            }
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
        let password = {
            if state.settings_wrapper.settings.master_password {
                password
            } else {
                BASE64_STANDARD.encode(password)
            }
        };
        println!("Added account: {}", password);
        if let Some(conn) = state.sql.as_ref() {
            println!("Added account: {:#?}", email);
            Account::new(-1, email, username, password, imap_host, imap_port, None)
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
fn update_account(state: tauri::State<AppState>, mut account: Account) -> Result<(), String> {
    if let Ok(state) = state.0.lock() {
        // let password = {
        if state.settings_wrapper.settings.master_password {
        } else {
            account.password = BASE64_STANDARD.encode(account.password)
        }
        // };
        if let Some(conn) = state.sql.as_ref() {
            account.update(conn).map_err(|e| e.to_string())
        } else {
            Err("Failed to get database connection".to_string())
        }
    } else {
        Err("Failed to lock state".to_string())
    }
}

#[tauri::command]
fn delete_account(state: tauri::State<AppState>, id: i64) -> Result<(), String> {
    if let Ok(state) = state.0.lock() {
        if let Some(conn) = state.sql.as_ref() {
            conn.delete_account(id).map_err(|e| e.to_string())
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
    state.0.lock().unwrap().save();
    app.exit(0);
    Ok(())
}

#[tauri::command]
fn ready(state: tauri::State<AppState>) {
    println!("Frontend is ready");
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
        state
            .events
            .retain(|event| !matches!(event, FrontendEvent::Log(_)));
        Ok(logs)
    } else {
        Err("Failed to lock state".to_string())
    }
}

fn frontend_event_dispatch_loop(app_state: Arc<Mutex<Shareble>>, handle: AppHandle) {
    tauri::async_runtime::spawn(async move {
        loop {
            if app_state.lock().unwrap().logout {
                break;
            }
            if let Ok(mut app_state) = app_state.lock() {
                if !app_state.frontend_ready {
                    continue;
                }
                let mut events = app_state.events.clone();
                app_state.events.clear();
                // drop(app_state);
                // remove duplicate action events
                events.dedup_by(|a, b| {
                    if let (FrontendEvent::Action(a), FrontendEvent::Action(b)) = (a, b) {
                        a.action == b.action && a.payload == b.payload
                    } else {
                        false
                    }
                });
                if !events.is_empty() {
                    println!("Sending events to frontend: {:#?}", events);
                    // weird thing the third event does not fire
                    for event in events {
                        match event {
                            FrontendEvent::Log(log) => {
                                handle.get_window("main").unwrap().emit("log", log).unwrap()
                            }
                            FrontendEvent::Action(action) => handle
                                .get_window("main")
                                .unwrap()
                                .emit("action", action)
                                .unwrap(),
                            FrontendEvent::Notify(notify) => handle
                                .get_window("main")
                                .unwrap()
                                .emit("notify", notify)
                                .unwrap(),
                        }
                    }
                }
            }
            tokio::time::sleep(tokio::time::Duration::from_millis(1000)).await;
        }
    });
}

#[tauri::command]
fn get_settings(state: tauri::State<AppState>) -> Result<settings::Settings, String> {
    if let Ok(state) = state.0.lock() {
        Ok(state.settings_wrapper.settings.clone())
    } else {
        Err("Failed to lock state".to_string())
    }
}

#[tauri::command]
fn add_event(state: tauri::State<AppState>, event_type: String, payload: String) {
    if let Ok(mut state) = state.0.lock() {
        let event = match event_type.as_str() {
            "log" => FrontendEvent::Log(LoggerPayload {
                message: payload,
                log_type: app_state::LoggerType::Info,
            }),
            "action" => FrontendEvent::Action(app_state::ActionPayload {
                action: payload,
                payload: "".to_string(),
            }),
            _ => return,
        };
        state.events.push(event);
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            get_account,
            get_accounts,
            add_account,
            logout,
            ready,
            get_top_emails,
            get_emails,
            get_email,
            fetch_logs,
            get_settings,
            add_event,
            delete_account,
            update_account
        ])
        .setup(|app| {
            let handle = app.handle();
            app.manage(AppState(Arc::new(Mutex::new(Shareble::new(
                handle.path_resolver().app_data_dir().unwrap(),
            )))));

            let app_state: Arc<Mutex<Shareble>> = Arc::clone(&app.state::<AppState>().inner().0);

            frontend_event_dispatch_loop(app_state, handle);

            let mut join_handles: Vec<tauri::async_runtime::JoinHandle<()>> = vec![];
            for account in app.state::<AppState>().get_accounts().unwrap() {
                let app_state: Arc<Mutex<Shareble>> =
                    Arc::clone(&app.state::<AppState>().inner().0);

                let handle = tauri::async_runtime::spawn(async move {
                    imap(app_state, account).await;
                });
                join_handles.push(handle);
            }
            let app_state: Arc<Mutex<Shareble>> = Arc::clone(&app.state::<AppState>().inner().0);
            tauri::async_runtime::spawn(async move {
                for handle in join_handles {
                    handle.await.unwrap();
                }
                println!("All imap tasks finished");
                app_state.lock().unwrap().backend_closed = true;
            });
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
