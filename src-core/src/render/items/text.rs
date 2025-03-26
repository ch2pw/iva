use ab_glyph::{FontRef, VariableFont};
use image::{Rgba, RgbaImage};
use imageproc::drawing::draw_text_mut;

use crate::{
    SCREEN_HEIGHT, SCREEN_WIDTH, render::Draw, parse::items::text::TextProps,
    types::ParsedItemProps,
};

impl Draw for ParsedItemProps<TextProps> {
    fn draw(&self, _time: u64) -> RgbaImage {
        let mut img = RgbaImage::from_pixel(SCREEN_WIDTH, SCREEN_HEIGHT, Rgba([0, 0, 0, 0]));

        let mut font = FontRef::try_from_slice(include_bytes!(
            "../../../../src/assets/fonts/notosansjp.ttf"
        ))
        .unwrap();
        font.set_variation(b"wght", 600.0);
        draw_text_mut(
            &mut img,
            self.others.color,
            self.others.x,
            self.others.y,
            self.others.font_size * 5.0,
            &font,
            &self.others.text,
        );
        img
    }
}
