use std::{collections::HashMap, sync::Mutex};

use iva_core::types::Item;

mod commands;
mod menu;

#[derive(Default, Debug)]
struct AppState {
    layers: HashMap<i32, Vec<Item>>,
    time: u64,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .manage(Mutex::new(AppState::default()))
        .invoke_handler(tauri::generate_handler![
            commands::update_layers,
            commands::update_time,
            commands::render
        ])
        .setup(|app| {
            app.on_menu_event(|app, event| match event.id().as_ref() {
                "export-mp4" => {
                    menu::export_mp4(app.clone());
                }
                "export-gif" => {
                    menu::export_gif(app.clone());
                }
                "export-png" => {
                    menu::export_png(app.clone());
                }
                _ => {}
            });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("Error while running Iva");

    Ok(())
}
