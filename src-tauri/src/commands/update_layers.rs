use std::{collections::HashMap, sync::Mutex};

use iva_core::types::Item;
use tauri::Manager;

use crate::AppState;

#[tauri::command]
pub async fn update_layers(app: tauri::AppHandle, layers: HashMap<i32, Vec<Item>>) {
    app.state::<Mutex<AppState>>().lock().unwrap().layers = layers;
}
