use std::collections::HashMap;

use image::{EncodableLayout, ImageBuffer, imageops::overlay};
use imageproc::{drawing::draw_filled_rect_mut, rect::Rect};
use types::Item;
use wasm_bindgen::{Clamped, prelude::*};
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, ImageData, window};

mod types;

const SCREEN_WIDTH: u32 = 1980;
const SCREEN_HEIGHT: u32 = 1080;

type Rgba = image::Rgba<u8>;

#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();

    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let canvas = document
        .get_element_by_id("screen")
        .unwrap()
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .unwrap();
    let context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();
    canvas.set_width(SCREEN_WIDTH);
    canvas.set_height(SCREEN_HEIGHT);
    Ok(())
}

fn parse_color(color: &str) -> Rgba {
    let color = color.trim_start_matches('#');
    let r = u8::from_str_radix(&color[0..2], 16).unwrap();
    let g = u8::from_str_radix(&color[2..4], 16).unwrap();
    let b = u8::from_str_radix(&color[4..6], 16).unwrap();
    let a = color
        .get(6..8)
        .filter(|a| !a.is_empty())
        .map_or(255, |a| u8::from_str_radix(a, 16).unwrap());
    Rgba::from([r, g, b, a])
}

#[wasm_bindgen]
pub async fn render(items: JsValue, time: u64) {
    let items = serde_wasm_bindgen::from_value::<HashMap<String, Item>>(items).unwrap();
    let item = items.values().find(|item| item.kind == "rect").unwrap();
    let x1 = item.props["x"].as_f64().unwrap() as i64;
    let y1 = item.props["y"].as_f64().unwrap() as i64;
    let width = item.props["width"].as_f64().unwrap() as u32;
    let height = item.props["height"].as_f64().unwrap() as u32;
    let color = item.props["color"].as_str().unwrap();

    let black = Rgba::from([0, 0, 0, 255]);
    let color = parse_color(&color);

    let mut img = ImageBuffer::from_pixel(SCREEN_WIDTH, SCREEN_HEIGHT, black);
    let mut rect = ImageBuffer::from_pixel(width, height, black);
    draw_filled_rect_mut(&mut rect, Rect::at(0, 0).of_size(width, height), color);
    overlay(&mut img, &rect, x1, y1);

    let window = window().unwrap();
    let document = window.document().unwrap();
    let canvas = document
        .get_element_by_id("screen")
        .unwrap()
        .dyn_into::<HtmlCanvasElement>()
        .unwrap();
    let context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<CanvasRenderingContext2d>()
        .unwrap();
    let data = img.as_bytes();
    let image_data = ImageData::new_with_u8_clamped_array(Clamped(&data), SCREEN_WIDTH).unwrap();
    context.put_image_data(&image_data, 0.0, 0.0).unwrap();
}
