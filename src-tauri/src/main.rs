// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

extern crate diesel;

use std::fmt::Debug;

use std::sync::Mutex;
use log::{debug, info};
use diesel::prelude::*;
use reqwest::{header::HeaderMap, StatusCode};
use serde::de;
use serde::Deserialize;
use serde::{ser::SerializeMap, Serialize};
use fantastic_lamp::{establish_connection, AppState};
use fantastic_lamp::models::Config;
use uuid::Uuid;

mod models;
mod schema;

#[derive(Debug, Clone, Deserialize, Serialize)]
struct RequestResponse {
    body: String,
    headers: CustomHeaderMap,
    status: CustomStatusCode,
}

#[derive(Debug, Deserialize, Serialize)]
struct RequestParameter {
    uuid: String,
    enabled: bool,
    key: String,
    value: String
}


#[derive(Debug, Deserialize, Serialize)]
struct FullTabdata {
    uuid: String,
    data: Tabdata,
    saved_data: Option<Tabdata>
}

#[derive(Debug, Deserialize, Serialize)]
struct Tabdata {
    name: String,
    url: String,
    response: Option<RequestResponse>,
    parameters: Vec<RequestParameter>
}

struct CustomStatusCode(StatusCode);
struct CustomHeaderMap(HeaderMap);

impl Debug for CustomStatusCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.as_str())
    }
}

impl Clone for CustomStatusCode {
    fn clone(&self) -> Self {
        CustomStatusCode(self.0.clone())
    }
}

impl Serialize for CustomStatusCode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_u16(self.0.as_u16())
    }
}

impl<'de> de::Deserialize<'de> for CustomStatusCode {
    fn deserialize<D>(d: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        let s = u16::deserialize(d);
        Ok(CustomStatusCode(StatusCode::from_u16(s.unwrap()).unwrap()))
    }
}


impl<'de> de::Deserialize<'de> for CustomHeaderMap {
    fn deserialize<D>(d: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        // TODO: Properly deserialize headers
        // let s = CustomHeaderMap::deserialize(d);
        Ok(CustomHeaderMap(HeaderMap::new()))
        // let s = d.deserialize_map(d);
        // Ok(CustomHeaderMap(StatusCode::from_u16(s.unwrap()).unwrap()))
    }
}

impl Debug for CustomHeaderMap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl Clone for CustomHeaderMap {
    fn clone(&self) -> Self {
        CustomHeaderMap(self.0.clone())
    }
}

impl Serialize for CustomHeaderMap {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut map = serializer.serialize_map(Some(self.0.len()))?;
        for (k, v) in self.0.clone() {
            map.serialize_entry(&k.unwrap().as_str(), v.to_str().unwrap())?;
        }
        map.end()
    }
}

#[tauri::command]
async fn send_get_request(api_url: String) -> RequestResponse {
    info!("Run GET request {:?}", api_url);
    let client = reqwest::Client::new();
    let request = client
        .get(api_url)
        .header(reqwest::header::USER_AGENT, "TestApi/1.0")
        .send()
        .await
        .unwrap();

    debug!("GET response: {:?}", request);
    let headers = CustomHeaderMap(request.headers().clone());
    let status = CustomStatusCode(request.status().clone());
    let body = request.text().await.unwrap();

    RequestResponse {
        body,
        headers,
        status,
    }
}

#[tauri::command]
async fn send_post_request(api_url: String) -> RequestResponse {
    info!("Run POST request {:?}", api_url);
    let client = reqwest::Client::new();
    let request = client
        .post(api_url)
        .header(reqwest::header::USER_AGENT, "TestApi/1.0")
        .send()
        .await
        .unwrap();

    debug!("POST response: {:?}", request);
    let headers = CustomHeaderMap(request.headers().clone());
    let status = CustomStatusCode(request.status().clone());
    let body = request.text().await.unwrap();

    RequestResponse {
        body,
        headers,
        status,
    }
}

#[tauri::command]
async fn send_put_request(api_url: String) -> RequestResponse {
    info!("Run PUT request {:?}", api_url);
    let client = reqwest::Client::new();
    let request = client
        .put(api_url)
        .header(reqwest::header::USER_AGENT, "TestApi/1.0")
        .send()
        .await
        .unwrap();

    debug!("PUT response: {:?}", request);
    let headers = CustomHeaderMap(request.headers().clone());
    let status = CustomStatusCode(request.status().clone());
    let body = request.text().await.unwrap();

    RequestResponse {
        body,
        headers,
        status,
    }
}


#[tauri::command]
async fn send_delete_request(api_url: String) -> RequestResponse {
    info!("Run DELETE request {:?}", api_url);
    let client = reqwest::Client::new();
    let request = client
        .delete(api_url)
        .header(reqwest::header::USER_AGENT, "TestApi/1.0")
        .send()
        .await
        .unwrap();

    debug!("DELETE response: {:?}", request);
    let headers = CustomHeaderMap(request.headers().clone());
    let status = CustomStatusCode(request.status().clone());
    let body = request.text().await.unwrap();

    RequestResponse {
        body,
        headers,
        status,
    }
}

// Get the config from db
// if config doesn't exist, create a new one
fn get_latest_config() -> Config {
    use fantastic_lamp::schema::config::dsl::*;
    let conn = &mut establish_connection();

    debug!("Getting log dates");
    let latest_config: Vec<Config> = config
        .select(Config::as_select())
        .load(conn)
        .expect("Expected to get all daily logs");

    debug!("{:?}", latest_config);

    if latest_config.len() == 0 {
        // No config found so init a new one
        let init_config = Config {
            uuid: Uuid::new_v4().to_string(),
            config_data: String::from("{}")
        };

        diesel::insert_into(config)
            .values(&init_config)
            .execute(conn)
            .unwrap();

        return init_config;
    }

    return latest_config.first().unwrap().clone();
}

#[tauri::command]
fn save_session(session_data: String, config: tauri::State<ConfigState>) {
    debug!("Save session: {:?}, data: {}", &config.0.config, session_data);
    let datas: Vec<FullTabdata> = serde_json::from_str(session_data.as_str()).unwrap();
    debug!("{:?}", datas);
    // TODO: Save session
}

struct ConfigState(AppState);

fn main() {
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("debug"));

    let config = get_latest_config();

    debug!("{:?}", config);

    info!("Starting Tauri backend.");
    tauri::Builder::default()
        .manage(ConfigState(AppState {
            config: Mutex::new(config),
        }))
        .invoke_handler(tauri::generate_handler![
            send_get_request,
            send_post_request,
            send_put_request,
            send_delete_request,
            save_session
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
