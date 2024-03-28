// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app;
mod database;
mod imap;
mod ipc;
mod settings;

use app::{state::Shareble, AppState};
use ipc::events::frontend_event_dispatch_loop;
use std::sync::{Arc, Mutex};
use tauri::Manager;

fn main() {
    simple_logger::SimpleLogger::new()
        .with_level(log::LevelFilter::Debug)
        .init()
        .unwrap();

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            ipc::commands::get_account,
            ipc::commands::get_accounts,
            ipc::commands::add_account,
            ipc::commands::logout,
            ipc::commands::ready,
            ipc::commands::get_top_emails,
            ipc::commands::get_emails,
            ipc::commands::get_email,
            ipc::commands::fetch_logs,
            ipc::commands::get_settings,
            ipc::commands::set_settings,
            ipc::commands::add_event,
            ipc::commands::delete_account,
            ipc::commands::update_account,
            ipc::commands::set_session_master_password,
            ipc::commands::start_all_imap_threads,
            ipc::commands::start_specific_imap_thread,
            ipc::commands::stop_specific_imap_thread
        ])
        .setup(|app| {
            let handle = app.handle();
            app.manage(AppState(Arc::new(Mutex::new(Shareble::new(
                handle.path_resolver().app_data_dir().unwrap(),
            )))));

            let app_state: Arc<Mutex<Shareble>> = Arc::clone(&app.state::<AppState>().inner().0);

            frontend_event_dispatch_loop(app_state, handle);

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
