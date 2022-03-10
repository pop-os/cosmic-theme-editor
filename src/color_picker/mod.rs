use anyhow::Result;
pub use exact::*;
use palette::rgb::Srgb;

mod exact;

pub trait ColorPicker {
    fn pick_color(
        &self,
        color: Srgb,
        contrast: f32,
        grayscale: bool,
        lighten: bool,
    ) -> Result<Srgb>;
}
