use std::{collections::HashMap, time::Instant};

use image::{Rgba, RgbaImage, imageops::overlay};
use render::{filters::into_applicable, into_drawable};
use types::Item;

pub mod parse;
pub mod render;

pub mod types;

const SCREEN_WIDTH: u32 = 1920;
const SCREEN_HEIGHT: u32 = 1080;

pub fn render(layers: &HashMap<i32, Vec<Item>>, time: u64) -> RgbaImage {
    let timer = Instant::now();
    let mut image = RgbaImage::from_pixel(SCREEN_WIDTH, SCREEN_HEIGHT, Rgba([0, 0, 0, 255]));

    for layer in 1..=50 {
        let item = layers.get(&layer).and_then(|items| {
            items
                .iter()
                .find(|item| item.props.common.time.contains(time))
        });

        let Some(item) = item else {
            continue;
        };
        
        let mut item_image = into_drawable(item.props.clone()).draw(time);
        for filter in &item.filters {
            into_applicable(filter.props.clone()).apply(time, &mut item_image);
        }
        overlay(&mut image, &item_image, 0, 0);
    }

    println!("Rendered in {:?}", timer.elapsed());

    image
}
