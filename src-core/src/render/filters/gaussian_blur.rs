use image::RgbaImage;

use crate::{
    render::Apply, parse::filters::gaussian_blur::GaussianBlurProps, types::ParsedFilterProps,
};

impl Apply for ParsedFilterProps<GaussianBlurProps> {
    fn apply(&self, _time: u64, image: &mut RgbaImage) {
        *image = imageproc::filter::gaussian_blur_f32(image, self.others.radius);
    }
}
