use super::ColorPicker;
use crate::util::SRGBA;
use anyhow::{bail, Result};
use float_cmp::approx_eq;
use palette::{Clamp, IntoColor, Lch, RelativeContrast};

#[derive(Debug, Default)]
pub struct Exact();

impl ColorPicker for Exact {
    fn pick_color(
        &self,
        color: SRGBA,
        contrast: f32,
        grayscale: bool,
        lighten: bool,
    ) -> Result<SRGBA> {
        let mut lch_color: Lch = (*color).into_color();

        // set to grayscale
        if grayscale {
            lch_color.chroma = 0.0;
        }

        // lighten or darken
        // TODO closed form solution using Lch color space contrast formula?
        // for now do binary search...
        let (min, max) = if lighten {
            (lch_color.l, 100.0)
        } else {
            (0.0, lch_color.l)
        };
        let (mut l, mut r) = (min, max);

        for _ in 0..100 {
            let cur_guess_lightness = (l + r) / 2.0;
            let mut cur_guess = lch_color;
            cur_guess.l = cur_guess_lightness;
            let cur_contrast = color.get_contrast_ratio(&cur_guess.into_color());
            if approx_eq!(f32, contrast, cur_contrast, ulps = 4) {
                lch_color = cur_guess;
                break;
            } else if contrast > cur_contrast {
                l = cur_guess_lightness;
            } else {
                r = cur_guess_lightness;
            }
        }

        // clamp to valid value in range
        lch_color.clamp_self();

        // verify contrast
        let actual_contrast = color.get_contrast_ratio(&lch_color.into_color());
        if !approx_eq!(f32, contrast, actual_contrast, ulps = 4) {
            bail!(
                "Failed to derive color with contrast {} from {}",
                contrast,
                color
            );
        }

        Ok(SRGBA(lch_color.into_color()))
    }
}
