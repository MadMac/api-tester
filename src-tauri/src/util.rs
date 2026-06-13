use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use reqwest::{header::HeaderMap, StatusCode};
use serde::Deserialize;
use serde::Serialize;
use std::sync::Mutex;

// Custom serialization for Option<StatusCode>
pub mod status_serializer {
    use super::StatusCode;
    use serde::{Deserialize, Deserializer, Serializer};
    use serde_json::Value;
    
    pub fn serialize<S>(status: &Option<StatusCode>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match status {
            Some(s) => serializer.collect_str(&s.as_str()),
            None => serializer.serialize_none(),
        }
    }
    
    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<StatusCode>, D::Error>
    where
        D: Deserializer<'de>,
    {
        use serde::de::Error;
        
        // Try to deserialize as Value first to handle both string and integer
        let value: Option<Value> = Option::deserialize(deserializer)?;
        match value {
            Some(Value::String(s)) => {
                let code: u16 = s.parse().map_err(Error::custom)?;
                Ok(Some(StatusCode::from_u16(code).map_err(Error::custom)?))
            },
            Some(Value::Number(n)) => {
                if let Some(code) = n.as_u64() {
                    Ok(Some(StatusCode::from_u16(code as u16).map_err(Error::custom)?))
                } else {
                    Err(Error::custom("Expected positive integer for status code"))
                }
            },
            Some(other) => Err(Error::custom(format!("Expected string or number for status code, got {:?}", other))),
            None => Ok(None),
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RequestResponse {
    pub body: String,
    #[serde(with = "http_serde::header_map")]
    pub headers: HeaderMap,
    #[serde(with = "status_serializer")]
    pub status: Option<StatusCode>,
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
    pub response: Option<RequestResponse>,
    pub parameters: Vec<RequestParameter>,
    pub headers: Vec<RequestHeader>,
    pub body: String,
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
