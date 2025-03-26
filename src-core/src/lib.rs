use std::{collections::HashMap, time::Instant};

use render::{Apply, Draw};
use image::{Rgba, RgbaImage, imageops::overlay};
use parse::{
    filters::gaussian_blur::GaussianBlurProps,
    items::{circle::CircleProps, rect::RectProps, text::TextProps},
};
use types::{Item, ParsedFilterProps, ParsedItemProps};

pub mod render;
pub mod parse;

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
        if let Some(item) = item {
            let mut item_image = match item.kind.as_str() {
                "rect" => ParsedItemProps::<RectProps>::from(item.props.clone()).draw(time),
                "circle" => ParsedItemProps::<CircleProps>::from(item.props.clone()).draw(time),
                "text" => ParsedItemProps::<TextProps>::from(item.props.clone()).draw(time),
                "image" => RgbaImage::from_pixel(0, 0, Rgba([0, 0, 0, 0])),
                "video" => RgbaImage::from_pixel(0, 0, Rgba([0, 0, 0, 0])),
                "audio" => RgbaImage::from_pixel(0, 0, Rgba([0, 0, 0, 0])),
                _ => RgbaImage::from_pixel(0, 0, Rgba([0, 0, 0, 0])),
            };
            for filter in &item.filters {
                match filter.kind.as_str() {
                    "gaussianBlur" => {
                        ParsedFilterProps::<GaussianBlurProps>::from(filter.props.clone())
                            .apply(time, &mut item_image)
                    }
                    _ => {}
                }
            }
            overlay(&mut image, &item_image, 0, 0);
        }
    }

    println!("Rendered in {:?}", timer.elapsed());

    image
}
