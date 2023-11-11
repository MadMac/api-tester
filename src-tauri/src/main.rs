// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use log::{debug, info};

#[tauri::command]
async fn send_get_request(api_url: String) -> String {
    info!("Run GET request {:?}", api_url);
    let client = reqwest::Client::new();
    let body = client
        .get(api_url)
        .header(reqwest::header::USER_AGENT, "TestApi/1.0")
        .send()
        .await
        .unwrap();

    debug!("GET response: {:?}", body);
    return body.text().await.unwrap();
}

#[tauri::command]
async fn send_post_request(api_url: String) -> String {
    info!("Run POST request {:?}", api_url);
    let client = reqwest::Client::new();
    let body = client
        .post(api_url)
        .header(reqwest::header::USER_AGENT, "TestApi/1.0")
        .send()
        .await
        .unwrap();

    debug!("POST response: {:?}", body);
    return body.text().await.unwrap();
}

#[tauri::command]
async fn send_put_request(api_url: String) -> String {
    info!("Run PUT request {:?}", api_url);
    let client = reqwest::Client::new();
    let body = client
        .put(api_url)
        .header(reqwest::header::USER_AGENT, "TestApi/1.0")
        .send()
        .await
        .unwrap();

    debug!("PUT response: {:?}", body);
    return body.text().await.unwrap();
}

fn main() {
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("debug"));

    info!("Starting Tauri backend.");
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            send_get_request,
            send_post_request,
            send_put_request
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
