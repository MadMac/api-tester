use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use reqwest::{header::HeaderMap, Method, StatusCode};
use serde::Deserialize;
use serde::Serialize;
use serde_nested_with::serde_nested;
use std::sync::Mutex;

#[serde_nested]
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RequestResponse {
    pub body: String,
    #[serde(with = "http_serde::header_map")]
    pub headers: HeaderMap,
    #[serde_nested(sub = "StatusCode", serde(with = "http_serde::status_code"))]
    pub status: Option<StatusCode>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RequestResponseTest {
    pub body: String,
    #[serde(with = "http_serde::header_map")]
    pub headers: HeaderMap,
    #[serde(with = "http_serde::status_code")]
    pub status: StatusCode,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct RequestParameter {
    pub uuid: String,
    pub enabled: bool,
    pub key: String,
    pub value: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct RequestHeader {
    pub uuid: String,
    pub enabled: bool,
    pub key: String,
    pub value: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct FullTabdata {
    pub uuid: String,
    pub data: Tabdata,
    pub saved_data: Option<Tabdata>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Tabdata {
    pub name: String,
    pub url: String,
    #[serde(rename = "requestType")]
    pub request_type: String,
    pub response: Option<RequestResponseTest>,
    pub parameters: Vec<RequestParameter>,
    pub headers: Vec<RequestHeader>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ConfigData {
    pub last_session: String,
}

pub struct ConfigState(pub AppState);

pub struct AppState {
    pub config: Mutex<ConfigData>,
}

pub fn establish_connection() -> SqliteConnection {
    SqliteConnection::establish("session_storage.db")
        .unwrap_or_else(|_| panic!("Error connecting to database"))
}
