use crate::database::account::{Account, AccountTable};

use super::AppState;

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