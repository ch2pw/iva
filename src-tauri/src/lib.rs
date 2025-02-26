use std::{collections::HashMap, io::Cursor};

use base64::prelude::*;
use image::{ImageBuffer, Rgb};
use types::Item;
use uuid::Uuid;

mod types;

#[tauri::command]
async fn render(items: HashMap<Uuid, Item>, time: u64) -> Result<String, String> {
    let width = 1980;
    let height = 1080;

    let item = items.values().find(|item| item.kind == "rect").unwrap();
    println!("{:?}", item);
    let x1 = item.props["x"].as_int();
    let y1 = item.props["y"].as_int();
    let x2 = x1 + item.props["width"].as_int();
    let y2 = y1 + item.props["height"].as_int();

    let mut img: ImageBuffer<Rgb<u8>, _> = ImageBuffer::from_pixel(width, height, Rgb([255, 255, 255]));

    let red = Rgb([255, 0, 0]);
    for x in x1..x2 {
        for y in y1..y2 {
            img.put_pixel(x as u32, y as u32, red);
        }
    }

    let mut bytes = Vec::new();
    img.write_to(&mut Cursor::new(&mut bytes), image::ImageFormat::Png).unwrap();

    Ok(BASE64_STANDARD.encode(bytes))
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![render])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
