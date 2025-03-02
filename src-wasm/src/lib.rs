use std::collections::HashMap;

use iva_core::types::Item;
use wasm_bindgen::{Clamped, prelude::*};
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, ImageData, window};

const SCREEN_WIDTH: u32 = 1980;
const SCREEN_HEIGHT: u32 = 1080;

#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();

    let window = window().unwrap();
    let document = window.document().unwrap();
    let canvas = document
        .get_element_by_id("screen")
        .unwrap()
        .dyn_into::<HtmlCanvasElement>()
        .unwrap();
    canvas.set_width(SCREEN_WIDTH);
    canvas.set_height(SCREEN_HEIGHT);
    Ok(())
}

fn context() -> CanvasRenderingContext2d {
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
    context
}

#[wasm_bindgen]
pub fn render(layers: JsValue, time: u64) {
    // JSON keys are always strings, so we need to convert them to integers.
    let layers = serde_wasm_bindgen::from_value::<HashMap<String, Vec<Item>>>(layers)
        .unwrap()
        .iter()
        .map(|(k, v)| (k.parse::<i32>().unwrap(), v.clone()))
        .collect::<HashMap<_, _>>();

    let img = iva_core::render(&layers, time);

    let data = img.as_raw();
    let image_data = ImageData::new_with_u8_clamped_array(Clamped(data), SCREEN_WIDTH).unwrap();
    context().put_image_data(&image_data, 0.0, 0.0).unwrap();
}
