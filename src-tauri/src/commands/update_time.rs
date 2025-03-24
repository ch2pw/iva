use std::sync::Mutex;

use tauri::Manager;

use crate::AppState;

#[tauri::command]
pub async fn update_time(app: tauri::AppHandle, time: u64) {
    app.state::<Mutex<AppState>>().lock().unwrap().time = time;
}
