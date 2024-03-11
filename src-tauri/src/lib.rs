extern crate diesel;
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
pub mod models;
pub mod schema;
use std::sync::Mutex;
use models::Config;

pub struct AppState {
    pub config: Mutex<Config>,
}

pub fn establish_connection() -> SqliteConnection {
    SqliteConnection::establish("session_storage.db")
        .unwrap_or_else(|_| panic!("Error connecting to database"))
}
