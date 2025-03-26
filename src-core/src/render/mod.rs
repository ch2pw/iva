use image::RgbaImage;

use crate::parse::ControlPoint;

pub mod filters;
pub mod items;

pub trait Draw {
    fn draw(&self, time: u64) -> RgbaImage;
}

pub trait Apply {
    fn apply(&self, time: u64, image: &mut RgbaImage);
}

pub fn interpolate(points: &[ControlPoint<f64>], progress: f64) -> f64 {
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
