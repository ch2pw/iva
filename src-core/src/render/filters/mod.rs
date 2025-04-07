use crate::{parse::filters::gaussian_blur::GaussianBlurProps, types::{FilterProps, ParsedFilterProps}};

use super::Apply;

pub mod gaussian_blur;

pub fn into_applicable(props: FilterProps) -> Box<dyn Apply> {
    match props.common.kind.as_str() {
        "gaussianBlur" => Box::new(ParsedFilterProps::<GaussianBlurProps>::from(props)),
        _ => panic!("Unknown filter type"),
    }
}
