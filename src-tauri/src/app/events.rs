use std::sync::{Arc, Mutex};

use serde::Serialize;

use super::state::Shareble;

pub trait EventDispatcher {
    fn log<T: ToString>(&mut self, message: T, log_type: LoggerType);
    fn log_error<T: ToString>(&mut self, message: T) {
        self.log(message, LoggerType::Error);
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