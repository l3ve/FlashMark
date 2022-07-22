use super::webview_center;
use serde::Deserialize;
use tauri::{command, AppHandle};

#[derive(Debug, Deserialize)]
pub struct RequestBody {
    id: i32,
    name: String,
}

#[command]
pub fn log_operation(event: String, payload: Option<String>) {
    println!("{} {:?}", event, payload);
}

#[command]
pub fn perform_request(endpoint: String, body: RequestBody) -> String {
    println!("{} {:?} {:?}", endpoint, body.id, body.name);
    "message response".into()
}

#[command]
pub fn alert(app: AppHandle) {
    webview_center::new_webview(app);
}
