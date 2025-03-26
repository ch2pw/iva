use image::Rgba;

use crate::{parse::parse_color, types::UniqueProps};

pub struct TextProps {
    pub x: i32,
    pub y: i32,
    pub text: String,
    pub font_size: f32,
    pub color: Rgba<u8>,
}

impl From<UniqueProps> for TextProps {
    fn from(props: UniqueProps) -> Self {
        let x = props["x"].as_f64().unwrap() as _;
        let y = props["y"].as_f64().unwrap() as _;
        let text = props["text"].as_str().unwrap().to_string();
        let font_size = props["fontSize"].as_f64().unwrap() as _;
        let color = props["color"].as_str().unwrap();
        let color = parse_color(color);
        TextProps {
            x,
            y,
            text,
            font_size,
            color,
        }
    }
}
