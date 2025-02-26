use std::{collections::HashMap, io::Cursor};

use base64::prelude::*;
use image::{imageops::overlay, ImageBuffer, Rgb};
use imageproc::{drawing::draw_filled_rect_mut, rect::Rect};
use types::Item;
use uuid::Uuid;

mod types;

const SCREEN_WIDTH: u32 = 1980;
const SCREEN_HEIGHT: u32 = 1080;

fn parse_color(color: &str) -> Rgb<u8> {
    let color = color.trim_start_matches('#');
    let r = u8::from_str_radix(&color[0..2], 16).unwrap();
    let g = u8::from_str_radix(&color[2..4], 16).unwrap();
    let b = u8::from_str_radix(&color[4..6], 16).unwrap();
    Rgb([r, g, b])
}

#[tauri::command]
async fn render(items: HashMap<Uuid, Item>, time: u64) -> Result<String, String> {
    let item = items.values().find(|item| item.kind == "rect").unwrap();
    let x1 = item.props["x"].as_int() as i64;
    let y1 = item.props["y"].as_int() as i64;
    let width = item.props["width"].as_int() as u32;
    let height = item.props["height"].as_int() as u32;
    let color = item.props["color"].as_string();

    let black: Rgb<u8> = Rgb([0, 0, 0]);
    let color: Rgb<u8> = parse_color(&color);

    let mut img = ImageBuffer::from_pixel(SCREEN_WIDTH, SCREEN_HEIGHT, black);
    let mut rect = ImageBuffer::from_pixel(width, height, black);
    draw_filled_rect_mut(&mut rect, Rect::at(0, 0).of_size(width, height), color);
    overlay(&mut img, &rect, x1, y1);

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
