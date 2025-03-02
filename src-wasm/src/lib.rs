use std::collections::HashMap;

use ab_glyph::{FontRef, VariableFont};
use image::ImageBuffer;
use imageproc::{drawing::draw_text_mut, map::map_enumerated_pixels_mut_parallel};
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

    let window = window().unwrap();
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
    let x = item.props["x"].as_f64().unwrap() as u32;
    let y = item.props["y"].as_f64().unwrap() as u32;
    let width = item.props["width"].as_f64().unwrap() as u32;
    let height = item.props["height"].as_f64().unwrap() as u32;
    let color = item.props["color"].as_str().unwrap();
    let color = parse_color(color);

    if width == 0 || height == 0 {
        return;
    }

    // draw_filled_rect_mut(img, Rect::at(x, y).of_size(width, height), color);
    map_enumerated_pixels_mut_parallel(img, |px, py, pixel| {
        if (x..x + width).contains(&px) && (y..y + height).contains(&py) {
            color
        } else {
            pixel
        }
    });
}

fn render_circle(img: &mut ImageBuffer<Rgba, Vec<u8>>, item: &Item) {
    let x = item.props["x"].as_f64().unwrap() as u32;
    let y = item.props["y"].as_f64().unwrap() as u32;
    let radius = item.props["radius"].as_f64().unwrap() as u32;
    let color = item.props["color"].as_str().unwrap();
    let color = parse_color(color);

    // draw_filled_circle_mut(img, (x, y), radius, color);
    map_enumerated_pixels_mut_parallel(img, |px, py, pixel| {
        let dx = px - x;
        let dy = py - y;
        if dx * dx + dy * dy <= radius * radius {
            color
        } else {
            pixel
        }
    });
}

fn render_text(img: &mut ImageBuffer<Rgba, Vec<u8>>, item: &Item) {
    let x = item.props["x"].as_f64().unwrap() as i32;
    let y = item.props["y"].as_f64().unwrap() as i32;
    let text = item.props["text"].as_str().unwrap();
    let font_size = item.props["fontSize"].as_f64().unwrap() as f32;
    let color = item.props["color"].as_str().unwrap();
    let color = parse_color(color);

    let mut font = FontRef::try_from_slice(include_bytes!("../../src/assets/fonts/notosansjp.ttf"))
        .expect("Failed to load font.");
    font.set_variation(b"wght", 600.0);
    draw_text_mut(img, color, x, y, font_size * 5.0, &font, text);
}

#[wasm_bindgen]
pub fn render(layers: JsValue, time: u64) {
    let layers = serde_wasm_bindgen::from_value::<HashMap<String, Vec<Item>>>(layers).unwrap();

    let mut img = ImageBuffer::from_pixel(SCREEN_WIDTH, SCREEN_HEIGHT, Rgba::from([0, 0, 0, 255]));

    for layer in (1..=50).map(|i| i.to_string()) {
        let item = layers
            .get(&layer)
            .and_then(|items| items.iter().find(|item| item.time.contains(time)));
        if let Some(item) = item {
            match item.kind.as_str() {
                "rect" => render_rect(&mut img, item),
                "circle" => render_circle(&mut img, item),
                "text" => render_text(&mut img, item),
                "image" => {}
                "video" => {}
                "audio" => {}
                _ => {}
            }
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
    let data = img.as_raw();
    let image_data = ImageData::new_with_u8_clamped_array(Clamped(data), SCREEN_WIDTH).unwrap();
    context.put_image_data(&image_data, 0.0, 0.0).unwrap();
}
