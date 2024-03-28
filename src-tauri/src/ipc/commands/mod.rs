use tauri::AppHandle;

use crate::app::{state, AppState};

mod account;
mod email;
mod event;
mod imap;
mod settings;

pub use account::*;
pub use email::*;
pub use event::*;
pub use imap::*;
pub use settings::*;

#[tauri::command]
pub async fn logout(state: tauri::State<'_, AppState>, app: AppHandle) -> Result<(), String> {
    {
        let mut state = state.0.lock().unwrap();
        if state.logout {
            return Ok(());
        }
        state.logout = true;
        state.save();
    }

    let mut timeout: u8 = 0;
    loop {
        if state.0.lock().unwrap().imap_threads.is_empty() {
            break;
        } else {
            log::info!("waiting for imap threads to close: {}", timeout);
            if timeout > 10 {
                for imap_thread in &state.0.lock().unwrap().imap_threads {
                    imap_thread.handle.abort();
                }
                break;
            }
        }
        timeout += 1;
        tokio::time::sleep(tokio::time::Duration::from_millis(1000)).await;
    }
    app.exit(0);
    Ok(())
}

#[tauri::command]
pub fn ready(state: tauri::State<AppState>) {
    log::info!("Frontend is ready");
    if let Ok(mut state) = state.0.lock() {
        state.frontend_ready = true;
    }
}
