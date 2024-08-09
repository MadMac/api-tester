use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use serde::Deserialize;
use serde::Serialize;
use std::sync::Mutex;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ConfigData {
    pub last_session: String,
}

pub struct AppState {
    pub config: Mutex<ConfigData>,
}

pub fn establish_connection() -> SqliteConnection {
    SqliteConnection::establish("session_storage.db")
        .unwrap_or_else(|_| panic!("Error connecting to database"))
}
