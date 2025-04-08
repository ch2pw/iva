use crate::{
    parse::items::{circle::CircleProps, rect::RectProps, text::TextProps},
    types::{ItemProps, ParsedItemProps},
};

use super::Draw;

pub mod circle;
pub mod rect;
pub mod text;

pub fn into_drawable(props: ItemProps) -> Box<dyn Draw> {
    match props.common.kind.as_str() {
        "circle" => Box::new(ParsedItemProps::<CircleProps>::from(props)),
        "rect" => Box::new(ParsedItemProps::<RectProps>::from(props)),
        "text" => Box::new(ParsedItemProps::<TextProps>::from(props)),
        _ => panic!("Unknown item type"),
    }
}
