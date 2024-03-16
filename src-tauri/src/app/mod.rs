use std::sync::{Arc, Mutex};

use self::state::Shareble;

pub mod state;
pub mod imap_thread;
pub mod events;
pub mod accounts;

pub struct AppState(pub Arc<Mutex<Shareble>>);