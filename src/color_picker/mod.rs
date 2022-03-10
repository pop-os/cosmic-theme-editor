use anyhow::Result;
pub use exact::*;
use palette::rgb::Srgba;

mod exact;

pub trait ColorPicker {
    fn pick_color(
        &self,
        color: Srgba,
        contrast: f32,
        grayscale: bool,
        lighten: bool,
    ) -> Result<Srgba>;
}
