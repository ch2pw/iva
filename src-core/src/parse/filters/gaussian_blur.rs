use crate::types::UniqueProps;

pub struct GaussianBlurProps {
    pub radius: f32,
}

impl From<UniqueProps> for GaussianBlurProps {
    fn from(props: UniqueProps) -> Self {
        let radius = props["radius"].as_f64().unwrap() as _;
        GaussianBlurProps { radius }
    }
}
