use crate::app::{events::{ActionPayload, FrontendEvent, LoggerPayload, LoggerType}, AppState};

#[tauri::command]
pub fn fetch_logs(state: tauri::State<AppState>) -> Result<Vec<LoggerPayload>, String> {
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

#[tauri::command]
pub fn add_event(state: tauri::State<AppState>, event_type: String, payload: String) {
    if let Ok(mut state) = state.0.lock() {
        let event = match event_type.as_str() {
            "log" => FrontendEvent::Log(LoggerPayload {
                message: payload,
                log_type: LoggerType::Info,
            }),
            "action" => FrontendEvent::Action(ActionPayload {
                action: payload,
                payload: "".to_string(),
            }),
            _ => return,
        };
        state.events.push(event);
    }
}