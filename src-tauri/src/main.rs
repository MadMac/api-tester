// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

extern crate diesel;

mod models;
mod schema;
mod session_management;
mod util;

use std::collections::HashMap;

use diesel::prelude::*;
use log::error;
use log::{debug, info};
use models::Config;
use reqwest::{header::HeaderMap, Method};
use session_management::{init_session, save_session};
use std::sync::Mutex;
use util::{establish_connection, AppState, ConfigData, ConfigState, RequestResponse, Tabdata};
use uuid::Uuid;

#[tauri::command]
async fn send_get_request(tab_data: Tabdata) -> RequestResponse {
    info!("Run GET request {:?}", tab_data);

    // Add parameters to the URL
    let mut params_map = HashMap::new();
    for param in tab_data.parameters {
        if param.enabled {
            params_map.insert(param.key, param.value);
        }
    }
    let url = reqwest::Url::parse_with_params(tab_data.url.as_str(), &params_map).unwrap();

    let header_map = reqwest::header::HeaderMap::new();

    let response: RequestResponse = send_request(Method::GET, url, header_map).await;

    return response;
}

#[tauri::command]
async fn send_post_request(tab_data: Tabdata) -> RequestResponse {
    info!("Run POST request {:?}", tab_data);
    let url = reqwest::Url::parse(tab_data.url.as_str()).unwrap();

    let mut header_map = reqwest::header::HeaderMap::new();

    let response: RequestResponse = send_request(Method::POST, url, header_map).await;

    return response;
}

#[tauri::command]
async fn send_put_request(tab_data: Tabdata) -> RequestResponse {
    info!("Run PUT request {:?}", tab_data);
    let url = reqwest::Url::parse(tab_data.url.as_str()).unwrap();

    let mut header_map = reqwest::header::HeaderMap::new();

    let response: RequestResponse = send_request(Method::PUT, url, header_map).await;

    return response;
}

#[tauri::command]
async fn send_delete_request(tab_data: Tabdata) -> RequestResponse {
    info!("Run DELETE request {:?}", tab_data);
    let url = reqwest::Url::parse(tab_data.url.as_str()).unwrap();

    let mut header_map = reqwest::header::HeaderMap::new();

    let response: RequestResponse = send_request(Method::DELETE, url, header_map).await;

    return response;
}

async fn send_request(method: Method, url: reqwest::Url, headers: HeaderMap) -> RequestResponse {
    let client = reqwest::Client::new();

    let mut header_map = reqwest::header::HeaderMap::new();

    for header in headers {
        header_map.insert(header.0.unwrap().clone(), header.1.clone());
    }
    header_map.insert(reqwest::header::USER_AGENT, "TestAoi/1.0".parse().unwrap());

    let request: RequestResponse = match client
        .request(method.clone(), url)
        .headers(header_map)
        .send()
        .await
    {
        Ok(request) => {
            debug!("{:?} response: {:?}", method, request);
            let headers = request.headers().clone();
            let status = Some(request.status().clone());
            let body = request.text().await.unwrap();

            RequestResponse {
                body,
                headers,
                status,
            }
        }
        Err(_) => {
            let body = String::from("Error sending request");
            let headers = HeaderMap::new();
            let status = None;

            RequestResponse {
                body,
                headers,
                status,
            }
        }
    };

    return request;
}

// Get the config from db
// if config doesn't exist, create a new one
fn get_latest_config() -> Config {
    use schema::config::dsl::*;
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
        .plugin(tauri_plugin_shell::init())
        .manage(ConfigState(AppState {
            config: Mutex::new(config_data),
        }))
        .invoke_handler(tauri::generate_handler![
            send_get_request,
            send_post_request,
            send_put_request,
            send_delete_request,
            save_session,
            init_session
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
