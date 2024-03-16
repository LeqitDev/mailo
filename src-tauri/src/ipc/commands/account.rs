
use crate::{app::AppState, database::{Account, AccountTable}};
use base64::prelude::*;

#[tauri::command]
pub fn get_account(state: tauri::State<AppState>, id: i64) -> Result<Account, String> {
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
pub fn get_accounts(state: tauri::State<AppState>) -> Result<Vec<Account>, String> {
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
pub fn add_account(
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
pub fn update_account(state: tauri::State<AppState>, mut account: Account) -> Result<(), String> {
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
pub fn delete_account(state: tauri::State<AppState>, id: i64) -> Result<(), String> {
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