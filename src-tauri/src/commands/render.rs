use std::sync::Mutex;

use tauri::{Manager, ipc::Response};

use crate::AppState;

#[tauri::command]
pub async fn render(app: tauri::AppHandle) -> Response {
    let thread = tauri::async_runtime::spawn(async move {
        let state = app.state::<Mutex<AppState>>();
        let state = state.lock().unwrap();
        let layers = state.layers.clone();
        let time = state.time;
        let image = iva_core::render(&layers, time);
        Response::new(image.as_raw().to_vec())
    });
    thread.await.unwrap()
}
