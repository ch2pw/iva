use image::Rgba;

use crate::{parse::parse_color, types::UniqueProps};

pub struct CircleProps {
    pub x: i32,
    pub y: i32,
    pub radius: i32,
    pub color: Rgba<u8>,
}

impl From<UniqueProps> for CircleProps {
    fn from(props: UniqueProps) -> Self {
        let x = props["x"].as_f64().unwrap() as _;
        let y = props["y"].as_f64().unwrap() as _;
        let radius = props["radius"].as_f64().unwrap() as _;
        let color = props["color"].as_str().unwrap();
        let color = parse_color(color);
        CircleProps {
            x,
            y,
            radius,
            color,
        }
    }
}
