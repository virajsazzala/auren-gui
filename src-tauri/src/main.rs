#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use serde::Deserialize;
use serde_json::json;
use tauri::command;
use reqwest::Client;

#[derive(Deserialize)]
struct Hit {
    file_id: String,
}

#[command]
async fn search(query: String) -> Vec<String> {
    let client = Client::new();
    let url = "http://localhost:8080/search";
    let body = json!({ "query": query });

    let resp = client.post(url)
        .json(&body)
        .send()
        .await;

    match resp {
        Ok(r) => {
            if let Ok(text) = r.text().await {
                // try fmt1: [{ "file_id": "..." }, ...]
                if let Ok(hits) = serde_json::from_str::<Vec<Hit>>(&text) {
                    return hits.into_iter().map(|h| h.file_id).collect();
                }
                // try fmt2: ["id1", "id2", ...]
                if let Ok(strings) = serde_json::from_str::<Vec<String>>(&text) {
                    return strings;
                }
            }
            vec![]
        }
        Err(e) => {
            eprintln!("search request error: {}", e);
            vec![]
        }
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![search])
        .run(tauri::generate_context!())
        .expect("error while running tauri app");
}
