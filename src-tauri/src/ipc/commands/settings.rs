use crate::{app::AppState, settings};

#[tauri::command]
pub fn get_settings(state: tauri::State<AppState>) -> Result<settings::Settings, String> {
    if let Ok(state) = state.0.lock() {
        Ok(state.settings_wrapper.settings.clone())
    } else {
        Err("Failed to lock state".to_string())
    }
}

#[tauri::command]
pub fn set_settings(
    state: tauri::State<AppState>,
    settings: settings::Settings,
) -> Result<(), String> {
    if let Ok(mut state) = state.0.lock() {
        state.settings_wrapper.settings = settings;
        Ok(())
    } else {
        Err("Failed to lock state".to_string())
    }
}

#[tauri::command]
pub fn set_session_master_password(
    state: tauri::State<AppState>,
    password: String,
) -> Result<(), String> {
    if let Ok(mut state) = state.0.lock() {
        state.session_master_password = Some(password);
        Ok(())
    } else {
        Err("Failed to lock state".to_string())
    }
}