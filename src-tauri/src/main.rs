// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

extern crate diesel;

use std::fmt::Debug;

use diesel::prelude::*;
use fantastic_lamp::models::{Config, RequestTabsSessions};
use fantastic_lamp::{establish_connection, AppState, ConfigData};
use log::error;
use log::{debug, info};
use reqwest::{header::HeaderMap, StatusCode};
use serde::Deserialize;
use serde::Serialize;
use std::sync::Mutex;
use uuid::Uuid;

mod models;
mod schema;

#[derive(Debug, Clone, Deserialize, Serialize)]
struct RequestResponse {
    body: String,
    #[serde(with = "http_serde::header_map")]
    headers: HeaderMap,
    #[serde(with = "http_serde::status_code")]
    status: StatusCode,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
struct RequestResponseTest {
    body: String,
    #[serde(with = "http_serde::header_map")]
    headers: HeaderMap,
    #[serde(with = "http_serde::status_code")]
    status: StatusCode,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
struct RequestParameter {
    uuid: String,
    enabled: bool,
    key: String,
    value: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
struct FullTabdata {
    uuid: String,
    data: Tabdata,
    saved_data: Option<Tabdata>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
struct Tabdata {
    name: String,
    url: String,
    response: Option<RequestResponseTest>,
    parameters: Vec<RequestParameter>,
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
    let headers = request.headers().clone();
    let status = request.status().clone();
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
    let headers = request.headers().clone();
    let status = request.status().clone();
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
    let headers = request.headers().clone();
    let status = request.status().clone();
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
    let headers = request.headers().clone();
    let status = request.status().clone();
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
        .expect("Expected to get configs");

    debug!("{:?}", latest_config);

    if latest_config.len() == 0 {
        // No config found so init a new one

        // Init session id
        let init_config_data = ConfigData {
            last_session: Uuid::new_v4().to_string(),
        };

        let init_config = Config {
            uuid: Uuid::new_v4().to_string(),
            config_data: serde_json::to_string(&init_config_data).unwrap(),
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
    use crate::schema::requesttabs::dsl::*;
    use crate::schema::requesttabs_sessions::dsl as requesttabs_sessions_dsl;
    use crate::schema::sessions::dsl as sessions_dsl;
    let session = &config.0.config.lock().expect("Could not lock mutex");
    let conn = &mut establish_connection();

    debug!("Save session: {:?}, data: {}", session, session_data);
    let datas: Vec<FullTabdata> = match serde_json::from_str(session_data.as_str()) {
        Ok(val) => val,
        Err(err) => {
            error!("ERROR: {}", err);
            Vec::new()
        }
    };
    debug!("Parsed data: {:?}", datas);

    // Save session uuid if it doesn't exist yet
    let old_session_entry_query = sessions_dsl::sessions
        .filter(sessions_dsl::uuid.eq(session.last_session.clone()))
        .first::<models::Sessions>(conn);

    let old_session_entry = match old_session_entry_query {
        Ok(entry) => {
            info!("Session found: {:?}", entry);
            entry
        }
        Err(_) => {
            debug!("Add new session: {:?}", session.last_session);

            let new_session_entry = models::Sessions {
                uuid: session.last_session.clone(),
            };

            diesel::insert_into(sessions_dsl::sessions)
                .values(&new_session_entry)
                .execute(conn)
                .expect("Expect to add a new session entry");

            new_session_entry
        }
    };

    // Saving tab data to requesttabs
    for fullTabData in datas {
        debug!("{:?}", fullTabData);

        // Find old tabdata entry if it exists
        let old_entry_query = requesttabs
            .filter(uuid.eq(&fullTabData.uuid))
            .first::<models::RequestTabs>(conn);

        let old_entry = match old_entry_query {
            Ok(entry) => {
                // TODO: Do update
                debug!("Update data");
                entry
            }
            Err(_) => {
                debug!("Add entry");

                let saved_data: Option<String> = match &fullTabData.saved_data {
                    Some(val) => Some(serde_json::to_string(val).unwrap()),
                    None => None,
                };

                let new_entry = models::RequestTabs {
                    uuid: fullTabData.uuid.clone(),
                    tabdata: serde_json::to_string(&fullTabData.data.clone()).unwrap(),
                    tabdata_saved: saved_data,
                    saved_timestamp: None,
                };

                diesel::insert_into(requesttabs)
                    .values(&new_entry)
                    .execute(conn)
                    .expect("Expect entry to be added");

                info!("Added new entry: {:?}", new_entry);

                new_entry
            }
        };

        // Add to requesttabs_sessions
        let request_tabs_sessions = models::RequestTabsSessions {
            uuid: Uuid::new_v4().to_string(),
            requesttabs_uuid: old_entry.uuid,
            sessions_uuid: old_session_entry.uuid.clone(),
        };

        let old_requesttabs_essions_entry = requesttabs_sessions_dsl::requesttabs_sessions
            .filter(requesttabs_sessions_dsl::requesttabs_uuid.eq(&request_tabs_sessions.requesttabs_uuid))
            .filter(requesttabs_sessions_dsl::sessions_uuid.eq(&request_tabs_sessions.sessions_uuid))
            .first::<models::RequestTabsSessions>(conn);

        match old_requesttabs_essions_entry {
            Ok(_) => {
                debug!("Requesttabs_sessions found");
            }
            Err(_) => {
                diesel::insert_into(requesttabs_sessions_dsl::requesttabs_sessions)
                .values(&request_tabs_sessions)
                .execute(conn)
                .expect("Expect to add a new requesttabs_sessions entry");
            }
        };

    }

    // TODO: Save session
    // For all tabs:
    // Get tab if already saved
    // Save data
    // Make new save data if it doesn't exist yet
    // Add to requesttabs_sessions?
}

fn init_session_db(config: tauri::State<ConfigState>) {
    use crate::schema::sessions::dsl::*;
    let session = &config.0.config;
    let conn = &mut establish_connection();

    // Find old tabdata entry if it exists
    // let old_entry = sessions
    // .filter(uuid.eq())
    // .first::<models::RequestTabs>(conn);
}

struct ConfigState(AppState);

fn main() {
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("debug"));

    let config = get_latest_config();
    let config_data: ConfigData = match serde_json::from_str(config.config_data.as_str()) {
        Ok(val) => val,
        Err(err) => {
            error!("ERROR: {}", err);
            ConfigData {
                last_session: String::new(),
            }
        }
    };
    debug!("{:?}", config_data);

    info!("Starting Tauri backend.");
    tauri::Builder::default()
        .manage(ConfigState(AppState {
            config: Mutex::new(config_data),
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
