use tauri::AppHandle;

use crate::app::AppState;

mod account;
mod email;
mod imap;
mod settings;
mod event;

pub use account::*;
pub use email::*;
pub use imap::*;
pub use settings::*;
pub use event::*;

#[tauri::command]
pub async fn logout(state: tauri::State<'_, AppState>, app: AppHandle) -> Result<(), String> {
    let mut handles = vec![];
    {
        let mut state = state.0.lock().unwrap();
        state.logout = true;
        state.save();
        for imap_thread_idx in 0..state.imap_threads.len() - 1 {
            handles.push(state.imap_threads.remove(imap_thread_idx));
        }
    }

    println!("closing {} imap threads", handles.len());

    for handle in handles {
        println!("closing imap thread: {}", handle.account_id);
        handle.stop().await;
    }
    /* loop {
        if state.0.lock().unwrap().backend_closed {
            break;
        }
        tokio::time::sleep(tokio::time::Duration::from_millis(1000)).await;
    } */
    app.exit(0);
    Ok(())
}

#[tauri::command]
pub fn ready(state: tauri::State<AppState>) {
    println!("Frontend is ready");
    if let Ok(mut state) = state.0.lock() {
        state.frontend_ready = true;
    }
}