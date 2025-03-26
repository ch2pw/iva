use image::{Rgba, RgbaImage};
use imageproc::{drawing::draw_filled_rect_mut, rect::Rect};

use crate::{
    SCREEN_HEIGHT, SCREEN_WIDTH,
    render::{Draw, interpolate},
    parse::items::rect::RectProps,
    types::ParsedItemProps,
};

impl Draw for ParsedItemProps<RectProps> {
    fn draw(&self, time: u64) -> RgbaImage {
        let progress: f64 = (time - self.common.time.start) as f64
            / (self.common.time.end - self.common.time.start) as f64;
        let width = interpolate(&self.others.width, progress) as _;
        let height = interpolate(&self.others.height, progress) as _;
        let x = interpolate(&self.others.x, progress) as u32;
        let y = interpolate(&self.others.y, progress) as u32;
        if width == 0 || height == 0 {
            RgbaImage::from_pixel(0, 0, Rgba([0, 0, 0, 0]))
        } else {
            let mut img = RgbaImage::from_pixel(SCREEN_WIDTH, SCREEN_HEIGHT, Rgba([0, 0, 0, 0]));
            draw_filled_rect_mut(
                &mut img,
                Rect::at(x as i32, y as i32).of_size(width, height),
                self.others.color,
            );
            img
        }
    }
}
