// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::fmt::Debug;

use log::{debug, info};
use reqwest::{header::HeaderMap, StatusCode};
use serde::{ser::SerializeMap, Serialize};

#[derive(Debug, Clone, Serialize)]
struct RequestResponse {
    body: String,
    headers: CustomHeaderMap,
    status: CustomStatusCode,
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


fn main() {
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("debug"));

    info!("Starting Tauri backend.");
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            send_get_request,
            send_post_request,
            send_put_request,
            send_delete_request
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
