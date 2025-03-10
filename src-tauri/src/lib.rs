mod menu;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![])
        .setup(|app| {
            app.on_menu_event(|app_handle, event| match event.id().as_ref() {
                "export-mp4" => {
                    menu::export_mp4(app_handle.clone());
                }
                "export-gif" => {
                    menu::export_gif(app_handle);
                }
                "export-png" => {
                    menu::export_png(app_handle);
                }
                _ => {}
            });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("Error while running Iva");

    Ok(())
}
