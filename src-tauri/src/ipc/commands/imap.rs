use std::sync::{Arc, Mutex};

use crate::{app::{accounts::AccountAccessor, imap_thread::ImapThread, state::Shareble, AppState}, database::account::Account, imap::run_imap};

#[tauri::command]
pub fn start_all_imap_threads(state: tauri::State<AppState>) {
    start_imap_threads(state);
}

#[tauri::command]
pub fn start_specific_imap_thread(
    state: tauri::State<AppState>,
    account: Account,
) -> Result<(), String> {
    start_imap_thread(state.inner().0.clone(), account);
    Ok(())
}

#[tauri::command]
pub async fn stop_specific_imap_thread(state: tauri::State<'_, AppState>, id: i64) -> Result<(), String> {
    stop_imap_thread(state, id).await
}

pub async fn stop_imap_threads(state: tauri::State<'_, AppState>) {
    if let Ok(mut state) = state.0.lock() {
        for imap_thread_idx in 0..state.imap_threads.len() {
            state.imap_threads.remove(imap_thread_idx).stop().await;
        }
    }
}

async fn stop_imap_thread(state: tauri::State<'_, AppState>, id: i64) -> Result<(), String> {
    let imap_thread: Result<ImapThread, String> = if let Ok(mut state) = state.0.lock() {
        if let Some(imap_thread_idx) = state
            .imap_threads
            .iter()
            .position(|thread| thread.account_id == id)
        {
            Ok(state.imap_threads.remove(imap_thread_idx))
        } else {
            Err("Failed to find imap thread".to_string())
        }
    } else {
        Err("Failed to lock state".to_string())
    };

    match imap_thread {
        Ok(imap_thread) => {
            imap_thread.stop().await;
            Ok(())
        }
        Err(e) => Err(e),
    }
}

fn start_imap_threads(app_state: tauri::State<AppState>) {
    for account in app_state.get_accounts().unwrap() {
        start_imap_thread(app_state.inner().0.clone(), account);
    }
}

fn start_imap_thread(app_state: Arc<Mutex<Shareble>>, account: Account) {
    let id = account.id;
    if app_state.lock().unwrap().imap_threads.iter().any(|thread| thread.account_id == id) {
        return;
    }
    let cloned_app_state = Arc::clone(&app_state);

    let handle = tauri::async_runtime::spawn(async move {
        run_imap(cloned_app_state, account).await;
    });
    app_state
        .lock()
        .unwrap()
        .imap_threads
        .push((handle, id).into());
}