use std::sync::{Arc, Mutex};

use tauri::async_runtime::JoinHandle;

use crate::database::account::Account;

use super::state::Shareble;

pub struct ImapThread {
    pub handle: JoinHandle<()>,
    pub account_id: i64,
    pub stop: bool,
}

impl ImapThread {
    pub async fn stop(mut self) {
        self.stop = true;
        self.handle.await.unwrap();
    }
}

impl From<(JoinHandle<()>, i64)> for ImapThread {
    fn from((handle, account_id): (JoinHandle<()>, i64)) -> Self {
        Self {
            handle,
            account_id,
            stop: false,
        }
    }
}