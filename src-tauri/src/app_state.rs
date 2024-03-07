use std::{
    path::PathBuf,
    sync::{Arc, Mutex},
};

use rusqlite::Connection;
use serde::{Deserialize, Serialize};
use tauri::api::path::app_data_dir;

use crate::{
    database::{get_database, Account, AccountTable},
    settings::{Settings, SettingsTrait, SettingsWrapper},
};

pub struct AppState(pub Arc<Mutex<Shareble>>);

#[derive(Default)]
pub struct Shareble {
    pub sql: Option<Connection>,
    pub logout: bool,
    pub frontend_ready: bool,
    pub events: Vec<FrontendEvent>,
    pub backend_closed: bool,
    pub settings_wrapper: SettingsWrapper,
}

impl Shareble {
    pub fn new(mut path: PathBuf) -> Self {
        path.push("settings.json");
        let settings: SettingsWrapper = SettingsWrapper::load(path.clone());
        path.pop();
        match get_database(path) {
            Ok(conn) => Self {
                sql: Some(conn),
                logout: false,
                frontend_ready: false,
                events: Vec::new(),
                backend_closed: false,
                settings_wrapper: settings,
            },
            Err(e) => {
                println!("Failed to get database connection: {:#?}", e);
                Self {
                    sql: None,
                    logout: false,
                    frontend_ready: false,
                    events: Vec::new(),
                    backend_closed: false,
                    settings_wrapper: settings,
                }
            }
        }
    }

    pub fn save(&self) {
        self.settings_wrapper.save();
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

    fn push_notify<T: ToString>(&mut self, title: T, body: T) {
        self.events.push(FrontendEvent::Notify(NotifyPayload {
            title: title.to_string(),
            body: body.to_string(),
        }));
    }
}

pub trait AccountAccessor {
    fn get_accounts(&self) -> Result<Vec<Account>, rusqlite::Error>;
    fn get_account(&self, id: i64) -> Result<Account, rusqlite::Error>;
}

impl AccountAccessor for tauri::State<'_, AppState> {
    fn get_accounts(&self) -> Result<Vec<Account>, rusqlite::Error> {
        self.inner()
            .0
            .lock()
            .unwrap()
            .sql
            .as_ref()
            .unwrap()
            .get_accounts()
    }

    fn get_account(&self, id: i64) -> Result<Account, rusqlite::Error> {
        self.inner()
            .0
            .lock()
            .unwrap()
            .sql
            .as_ref()
            .unwrap()
            .get_account(id)
    }
}

pub trait EventDispatcher {
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
    fn notify<T: ToString>(&mut self, title: T, body: T);
}

impl EventDispatcher for Arc<Mutex<Shareble>> {
    fn log<T: ToString>(&mut self, message: T, log_type: LoggerType) {
        self.lock().unwrap().push_log(message, log_type);
    }
    fn action<T: ToString>(&mut self, action: T, payload: T) {
        self.lock().unwrap().push_action(action, payload);
    }
    fn notify<T: ToString>(&mut self, title: T, body: T) {
        self.lock().unwrap().push_notify(title, body);
    }
}

#[derive(Clone, Debug)]
pub enum FrontendEvent {
    Log(LoggerPayload),
    Action(ActionPayload),
    Notify(NotifyPayload),
}

trait EventPayload: Serialize + Clone {}

#[derive(Clone, Serialize, Debug, PartialEq)]
pub struct ActionPayload {
    pub action: String,
    pub payload: String,
}

#[derive(Clone, Serialize, Debug)]
pub enum LoggerType {
    Info,
    Warning,
    Error,
}

#[derive(Clone, Serialize, Debug)]
pub struct LoggerPayload {
    pub message: String,
    pub log_type: LoggerType,
}

#[derive(Clone, Serialize, Debug)]
pub struct NotifyPayload {
    pub title: String,
    pub body: String,
}
