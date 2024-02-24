use std::{path::PathBuf, sync::{Arc, Mutex}};

use rusqlite::Connection;
use serde::{Deserialize, Serialize};
use tauri::api::path::app_data_dir;

use crate::database::{get_database, Account, AccountTable};

pub struct AppState(pub Arc<Mutex<Shareble>>);

#[derive(Default)]
pub struct Shareble {
    pub sql: Option<Connection>,
    pub logout: bool,
    pub frontend_ready: bool,
    pub events: Vec<FrontendEvent>,
    pub backend_closed: bool,
    pub settings: Settings,
    settings_path: PathBuf,
}

impl Shareble {
    pub fn new(path: PathBuf) -> Self {
        let mut settings_path = path.clone();
        settings_path.push("settings.toml");
        let settings: Settings = confy::load_path(settings_path.clone()).unwrap();
        match get_database(path) {
            Ok(conn) => Self {
                sql: Some(conn),
                logout: false,
                frontend_ready: false,
                events: Vec::new(),
                backend_closed: false,
                settings,
                settings_path,
            },
            Err(e) => {
                println!("Failed to get database connection: {:#?}", e);
                Self {
                    sql: None,
                    logout: false,
                    frontend_ready: false,
                    events: Vec::new(),
                    backend_closed: false,
                    settings,
                    settings_path,
                }
            }
        }
    }

    pub fn save(&self) {
        confy::store_path(self.settings_path.clone(), self.settings.clone()).unwrap();
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

#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub struct Settings {
    pub master_password: bool,
}

pub trait AccountAccessor {
    fn get_accounts(&self) -> Result<Vec<Account>, rusqlite::Error>;
    fn get_account(&self, id: i64) -> Result<Account, rusqlite::Error>;
}

impl AccountAccessor for tauri::State<'_, AppState> {
    fn get_accounts(&self) -> Result<Vec<Account>, rusqlite::Error> {
        self.inner().0.lock().unwrap().sql.as_ref().unwrap().get_accounts()
    }

    fn get_account(&self, id: i64) -> Result<Account, rusqlite::Error> {
        self.inner().0.lock().unwrap().sql.as_ref().unwrap().get_account(id)
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
pub enum FrontendEvent {
    Log(LoggerPayload),
    Action(ActionPayload),
}

trait EventPayload: Serialize + Clone {
}

#[derive(Clone, Serialize, Debug)]
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