use super::ColorPicker;
use anyhow::{bail, Result};
use float_cmp::approx_eq;
use palette::{rgb::Srgb, Clamp, IntoColor, Lch, RelativeContrast};

#[derive(Debug, Default)]
pub struct Exact();

impl ColorPicker for Exact {
    fn pick_color(
        &self,
        color: Srgb,
        contrast: f32,
        grayscale: bool,
        lighten: bool,
    ) -> Result<Srgb> {
        let mut lch_color: Lch = color.into_color();
        dbg!(lch_color);

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
            dbg!((l, r));
            let cur_guess_lightness = (l + r) / 2.0;
            let mut cur_guess = lch_color;
            cur_guess.l = cur_guess_lightness;
            let cur_contrast = color.get_contrast_ratio(&cur_guess.into_color());
            dbg!((cur_guess_lightness, cur_contrast));
            if approx_eq!(f32, contrast, cur_contrast, ulps = 4) {
                lch_color = cur_guess;
                break;
            } else if contrast > cur_contrast {
                l = cur_guess_lightness;
            } else {
                r = cur_guess_lightness;
            }
        }

        dbg!(lch_color);
        // clamp to valid value in range
        lch_color.clamp_self();

        // verify contrast
        let actual_contrast = color.get_contrast_ratio(&lch_color.into_color());
        dbg!((contrast, actual_contrast));
        if !approx_eq!(f32, contrast, actual_contrast, ulps = 4) {
            bail!("Generated color does not match desired contrast exactly!");
        }

        Ok(lch_color.into_color())
    }
}
