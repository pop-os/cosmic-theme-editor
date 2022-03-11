use crate::util::SRGBA;
use anyhow::Result;
pub use exact::*;

mod exact;

pub trait ColorPicker {
    fn pick_color(
        &self,
        color: SRGBA,
        contrast: f32,
        grayscale: bool,
        lighten: Option<bool>,
    ) -> Result<SRGBA>;
}
