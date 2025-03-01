use std::collections::HashMap;

use image::{EncodableLayout, ImageBuffer, imageops::overlay};
use imageproc::{
    drawing::{draw_filled_circle_mut, draw_filled_rect_mut},
    rect::Rect,
};
use itertools::Itertools;
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
        .map_or(255, |a| u8::from_str_radix(a, 16).unwrap());
    Rgba::from([r, g, b, a])
}

fn render_rect(img: &mut ImageBuffer<Rgba, Vec<u8>>, item: &Item) {
    let x = item.props["x"].as_f64().unwrap() as i64;
    let y = item.props["y"].as_f64().unwrap() as i64;
    let width = item.props["width"].as_f64().unwrap() as u32;
    let height = item.props["height"].as_f64().unwrap() as u32;
    let color = item.props["color"].as_str().unwrap();
    let color = parse_color(&color);

    let mut rect: ImageBuffer<_, Vec<u8>> = ImageBuffer::new(width, height);
    draw_filled_rect_mut(&mut rect, Rect::at(0, 0).of_size(width, height), color);
    overlay(img, &rect, x, y);
}

fn render_circle(img: &mut ImageBuffer<Rgba, Vec<u8>>, item: &Item) {
    let x = item.props["x"].as_f64().unwrap() as i32;
    let y = item.props["y"].as_f64().unwrap() as i32;
    let radius = item.props["radius"].as_f64().unwrap() as i32;
    let color = item.props["color"].as_str().unwrap();
    let color = parse_color(&color);

    let mut circle: ImageBuffer<_, Vec<u8>> =
        ImageBuffer::new(radius as u32 * 2, radius as u32 * 2);
    draw_filled_circle_mut(&mut circle, (radius, radius), radius, color);
    overlay(img, &circle, (x - radius).into(), (y - radius).into());
}

#[wasm_bindgen]
pub async fn render(items: JsValue, time: u64) {
    let items = serde_wasm_bindgen::from_value::<HashMap<String, Item>>(items).unwrap();
    let items = items
        .values()
        .filter(|item| item.time.contains(time))
        .sorted_by_key(|item| item.layer)
        .collect::<Vec<_>>();

    let mut img = ImageBuffer::from_pixel(SCREEN_WIDTH, SCREEN_HEIGHT, Rgba::from([0, 0, 0, 255]));

    for item in items {
        match item.kind.as_str() {
            "rect" => render_rect(&mut img, item),
            "circle" => render_circle(&mut img, item),
            "text" => {}
            "image" => {}
            "video" => {}
            "audio" => {}
            _ => {}
        }
    }

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
