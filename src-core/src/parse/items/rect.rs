use image::Rgba;

use crate::{
    parse::{ControlPoint, parse_color},
    types::UniqueProps,
};

pub struct RectProps {
    pub x: Vec<ControlPoint<f64>>,
    pub y: Vec<ControlPoint<f64>>,
    pub width: Vec<ControlPoint<f64>>,
    pub height: Vec<ControlPoint<f64>>,
    pub color: Rgba<u8>,
}

impl From<UniqueProps> for RectProps {
    fn from(props: UniqueProps) -> Self {
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
        RectProps {
            x,
            y,
            width,
            height,
            color,
        }
    }
}
