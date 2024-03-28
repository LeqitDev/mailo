use std::path::PathBuf;

use rusqlite::Connection;

use crate::{
    database::get_database,
    settings::{SettingsTrait, SettingsWrapper},
};

use super::{
    events::{ActionPayload, FrontendEvent, LoggerPayload, LoggerType, NotifyPayload},
    imap_thread::ImapThread,
};

#[derive(Default)]
pub struct Shareble {
    pub sql: Option<Connection>,
    pub logout: bool,
    pub frontend_ready: bool,
    pub events: Vec<FrontendEvent>,
    pub backend_closed: bool,
    pub settings_wrapper: SettingsWrapper,
    pub imap_threads: Vec<ImapThread>,
    pub session_master_password: Option<String>,
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
                imap_threads: Vec::new(),
                session_master_password: None,
            },
            Err(e) => {
                log::error!("Failed to get database connection: {:#?}", e);
                Self {
                    sql: None,
                    logout: false,
                    frontend_ready: false,
                    events: Vec::new(),
                    backend_closed: false,
                    settings_wrapper: settings,
                    imap_threads: Vec::new(),
                    session_master_password: None,
                }
            }
        }
    }

    pub fn save(&self) {
        self.settings_wrapper.save();
    }

    pub fn push_log<T: ToString>(&mut self, message: T, log_type: LoggerType) {
        self.events.push(FrontendEvent::Log(LoggerPayload {
            message: message.to_string(),
            log_type,
        }));
    }

    pub fn push_action<T: ToString>(&mut self, action: T, payload: T) {
        self.events.push(FrontendEvent::Action(ActionPayload {
            action: action.to_string(),
            payload: payload.to_string(),
        }));
    }

    pub fn push_notify<T: ToString>(&mut self, title: T, body: T) {
        self.events.push(FrontendEvent::Notify(NotifyPayload {
            title: title.to_string(),
            body: body.to_string(),
        }));
    }
}
