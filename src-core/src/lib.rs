use std::collections::HashMap;

use ab_glyph::{FontRef, VariableFont};
use image::ImageBuffer;
use imageproc::{
    drawing::{draw_filled_circle_mut, draw_filled_rect_mut, draw_text_mut},
    rect::Rect,
};

pub mod types;

const SCREEN_WIDTH: u32 = 1920;
const SCREEN_HEIGHT: u32 = 1080;

type Rgba = image::Rgba<u8>;
pub trait Render {
    fn render(
        &self,
        img: &mut ImageBuffer<Rgba, Vec<u8>>,
        time_range: &types::TimeRange,
        time: u64,
    );
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

fn interpolate(points: &[ControlPoint<f64>], progress: f64) -> f64 {
    if points.len() == 1 {
        return points[0].value;
    }

    let prev = points.iter().rfind(|point| point.progress < progress);
    let next = points.iter().find(|point| point.progress >= progress);
    match (prev, next) {
        (Some(prev), Some(next)) => {
            let progress = (progress - prev.progress) / (next.progress - prev.progress);
            prev.value + (next.value - prev.value) * progress
        }
        (Some(prev), None) => prev.value,
        (None, Some(next)) => next.value,
        (None, None) => panic!("No control points"),
    }
}

pub struct ControlPoint<T> {
    pub progress: f64,
    pub value: T,
}

impl TryFrom<&serde_json::Value> for ControlPoint<f64> {
    type Error = &'static str;

    fn try_from(value: &serde_json::Value) -> Result<Self, Self::Error> {
        let progress = value["progress"]
            .as_f64()
            .ok_or("progress is not a number")?;
        let value = value["value"].as_f64().ok_or("value is not a number")?;
        Ok(ControlPoint { progress, value })
    }
}

pub struct IvaRect {
    pub x: Vec<ControlPoint<f64>>,
    pub y: Vec<ControlPoint<f64>>,
    pub width: Vec<ControlPoint<f64>>,
    pub height: Vec<ControlPoint<f64>>,
    pub color: Rgba,
}

impl From<&HashMap<String, serde_json::Value>> for IvaRect {
    fn from(props: &HashMap<String, serde_json::Value>) -> Self {
        let x = props["x"]
            .as_array()
            .unwrap()
            .iter()
            .map(|x| x.try_into().unwrap())
            .collect();
        let y = props["y"]
            .as_array()
            .unwrap()
            .iter()
            .map(|y| y.try_into().unwrap())
            .collect();
        let width = props["width"]
            .as_array()
            .unwrap()
            .iter()
            .map(|w| w.try_into().unwrap())
            .collect();
        let height = props["height"]
            .as_array()
            .unwrap()
            .iter()
            .map(|h| h.try_into().unwrap())
            .collect();
        let color = props["color"].as_str().unwrap();
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
    fn render(
        &self,
        img: &mut ImageBuffer<Rgba, Vec<u8>>,
        time_range: &types::TimeRange,
        time: u64,
    ) {
        let progress =
            (time - time_range.start) as f64 / (time_range.end - time_range.start) as f64;
        let width = interpolate(&self.width, progress) as _;
        let height = interpolate(&self.height, progress) as _;
        let x = interpolate(&self.x, progress) as _;
        let y = interpolate(&self.y, progress) as _;
        if width <= 0 || height <= 0 {
            return;
        }
        draw_filled_rect_mut(img, Rect::at(x, y).of_size(width, height), self.color);
    }
}

pub struct IvaCircle {
    pub x: i32,
    pub y: i32,
    pub radius: i32,
    pub color: Rgba,
}

impl From<&HashMap<String, serde_json::Value>> for IvaCircle {
    fn from(props: &HashMap<String, serde_json::Value>) -> Self {
        let x = props["x"].as_f64().unwrap() as _;
        let y = props["y"].as_f64().unwrap() as _;
        let radius = props["radius"].as_f64().unwrap() as _;
        let color = props["color"].as_str().unwrap();
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
    fn render(
        &self,
        img: &mut ImageBuffer<Rgba, Vec<u8>>,
        _time_range: &types::TimeRange,
        _time: u64,
    ) {
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

impl From<&HashMap<String, serde_json::Value>> for IvaText {
    fn from(props: &HashMap<String, serde_json::Value>) -> Self {
        let x = props["x"].as_f64().unwrap() as _;
        let y = props["y"].as_f64().unwrap() as _;
        let text = props["text"].as_str().unwrap().to_string();
        let font_size = props["fontSize"].as_f64().unwrap() as _;
        let color = props["color"].as_str().unwrap();
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
    fn render(
        &self,
        img: &mut ImageBuffer<Rgba, Vec<u8>>,
        _time_range: &types::TimeRange,
        _time: u64,
    ) {
        let mut font =
            FontRef::try_from_slice(include_bytes!("../../src/assets/fonts/notosansjp.ttf"))
                .unwrap();
        font.set_variation(b"wght", 600.0);
        draw_text_mut(
            img,
            self.color,
            self.x,
            self.y,
            self.font_size * 5.0,
            &font,
            &self.text,
        );
    }
}

pub fn render(layers: &HashMap<i32, Vec<types::Item>>, time: u64) -> ImageBuffer<Rgba, Vec<u8>> {
    let mut img = ImageBuffer::from_pixel(SCREEN_WIDTH, SCREEN_HEIGHT, Rgba::from([0, 0, 0, 255]));

    for layer in 1..=50 {
        let item = layers
            .get(&layer)
            .and_then(|items| items.iter().find(|item| item.time.contains(time)));
        if let Some(item) = item {
            match item.kind.as_str() {
                "rect" => IvaRect::from(&item.props).render(&mut img, &item.time, time),
                "circle" => IvaCircle::from(&item.props).render(&mut img, &item.time, time),
                "text" => IvaText::from(&item.props).render(&mut img, &item.time, time),
                "image" => {}
                "video" => {}
                "audio" => {}
                _ => {}
            }
        }
    }

    img
}
