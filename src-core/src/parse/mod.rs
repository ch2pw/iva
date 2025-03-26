use image::Rgba;

pub mod filters;
pub mod items;

pub struct ControlPoint<T> {
    pub progress: f64,
    pub value: T,
}

impl TryFrom<&serde_json::Value> for ControlPoint<f64> {
    type Error = &'static str;

    fn try_from(value: &serde_json::Value) -> Result<Self, Self::Error> {
        let progress = value["progress"]
            .as_f64()
            .ok_or("progress is not a number")?;
        let value = value["value"].as_f64().ok_or("value is not a number")?;
        Ok(ControlPoint { progress, value })
    }
}

fn parse_color(color: &str) -> Rgba<u8> {
    let color = color.trim_start_matches('#');
    let r = u8::from_str_radix(&color[0..2], 16).unwrap();
    let g = u8::from_str_radix(&color[2..4], 16).unwrap();
    let b = u8::from_str_radix(&color[4..6], 16).unwrap();
    let a = color
        .get(6..8)
        .map_or(255, |a| u8::from_str_radix(a, 16).unwrap());
    Rgba([r, g, b, a])
}
