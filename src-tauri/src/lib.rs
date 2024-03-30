extern crate diesel;
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
pub mod models;
pub mod schema;
use std::sync::Mutex;
use models::Config;
use serde::Deserialize;
use serde::Serialize;

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
