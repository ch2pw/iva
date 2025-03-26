use image::{Rgba, RgbaImage};
use imageproc::drawing::draw_filled_circle_mut;

use crate::{
    SCREEN_HEIGHT, SCREEN_WIDTH, render::Draw, parse::items::circle::CircleProps,
    types::ParsedItemProps,
};

impl Draw for ParsedItemProps<CircleProps> {
    fn draw(&self, _time: u64) -> RgbaImage {
        let mut img = RgbaImage::from_pixel(SCREEN_WIDTH, SCREEN_HEIGHT, Rgba([0, 0, 0, 0]));
        draw_filled_circle_mut(
            &mut img,
            (self.others.x, self.others.y),
            self.others.radius,
            self.others.color,
        );
        img
    }
}
