use crate::util::SRGBA;
use anyhow::Result;
pub use exact::*;

mod exact;

pub trait ColorPicker {
    fn pick_color(
        &self,
        color: SRGBA,
        contrast: Option<f32>,
        grayscale: bool,
        lighten: Option<bool>,
    ) -> Result<SRGBA>;

    fn pick_color_text(
        &self,
        color: SRGBA,
        grayscale: bool,
        lighten: Option<bool>,
    ) -> (SRGBA, Option<anyhow::Error>);

    fn pick_color_graphic(
        &self,
        color: SRGBA,
        contrast: f32,
        grayscale: bool,
        lighten: Option<bool>,
    ) -> (SRGBA, Option<anyhow::Error>);
}
