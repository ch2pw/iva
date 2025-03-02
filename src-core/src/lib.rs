use std::collections::HashMap;

use ab_glyph::{FontRef, VariableFont};
use image::ImageBuffer;
use imageproc::{
    drawing::{draw_filled_circle_mut, draw_filled_rect_mut, draw_text_mut},
    rect::Rect,
};

pub mod types;

type Rgba = image::Rgba<u8>;
pub trait Render {
    fn render(&self, img: &mut ImageBuffer<Rgba, Vec<u8>>);
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

pub struct IvaRect {
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
    pub color: Rgba,
}

impl From<&types::Item> for IvaRect {
    fn from(item: &types::Item) -> Self {
        let x = item.props["x"].as_f64().unwrap() as _;
        let y = item.props["y"].as_f64().unwrap() as _;
        let width = item.props["width"].as_f64().unwrap() as _;
        let height = item.props["height"].as_f64().unwrap() as _;
        let color = item.props["color"].as_str().unwrap();
        let color = parse_color(color);
        IvaRect {
            x,
            y,
            width,
            height,
            color,
        }
    }
}

impl Render for IvaRect {
    fn render(&self, img: &mut ImageBuffer<Rgba, Vec<u8>>) {
        if self.width == 0 || self.height == 0 {
            return;
        }
        draw_filled_rect_mut(
            img,
            Rect::at(self.x, self.y).of_size(self.width, self.height),
            self.color,
        );
    }
}

pub struct IvaCircle {
    pub x: i32,
    pub y: i32,
    pub radius: i32,
    pub color: Rgba,
}

impl From<&types::Item> for IvaCircle {
    fn from(item: &types::Item) -> Self {
        let x = item.props["x"].as_f64().unwrap() as _;
        let y = item.props["y"].as_f64().unwrap() as _;
        let radius = item.props["radius"].as_f64().unwrap() as _;
        let color = item.props["color"].as_str().unwrap();
        let color = parse_color(color);
        IvaCircle {
            x,
            y,
            radius,
            color,
        }
    }
}

impl Render for IvaCircle {
    fn render(&self, img: &mut ImageBuffer<Rgba, Vec<u8>>) {
        draw_filled_circle_mut(img, (self.x, self.y), self.radius, self.color);
    }
}

pub struct IvaText {
    pub x: i32,
    pub y: i32,
    pub text: String,
    pub font_size: f32,
    pub color: Rgba,
}

impl From<&types::Item> for IvaText {
    fn from(item: &types::Item) -> Self {
        let x = item.props["x"].as_f64().unwrap() as _;
        let y = item.props["y"].as_f64().unwrap() as _;
        let text = item.props["text"].as_str().unwrap().to_string();
        let font_size = item.props["fontSize"].as_f64().unwrap() as _;
        let color = item.props["color"].as_str().unwrap();
        let color = parse_color(color);
        IvaText {
            x,
            y,
            text,
            font_size,
            color,
        }
    }
}

impl Render for IvaText {
    fn render(&self, img: &mut ImageBuffer<Rgba, Vec<u8>>) {
        let mut font =
            FontRef::try_from_slice(include_bytes!("../../src/assets/fonts/notosansjp.ttf")).unwrap();
        font.set_variation(b"wght", 600.0);
        draw_text_mut(img, self.color, self.x, self.y, self.font_size * 5.0, &font, &self.text);
    }
}

pub fn render(layers: &HashMap<i32, Vec<types::Item>>, time: u64) -> ImageBuffer<Rgba, Vec<u8>> {
    let mut img = ImageBuffer::from_pixel(1980, 1080, Rgba::from([0, 0, 0, 255]));

    for layer in 1..=50 {
        let item = layers
            .get(&layer)
            .and_then(|items| items.iter().find(|item| item.time.contains(time)));
        if let Some(item) = item {
            match item.kind.as_str() {
                "rect" => IvaRect::from(item).render(&mut img),
                "circle" => IvaCircle::from(item).render(&mut img),
                "text" => IvaText::from(item).render(&mut img),
                "image" => {}
                "video" => {}
                "audio" => {}
                _ => {}
            }
        }
    }

    img
}
