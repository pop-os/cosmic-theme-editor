use super::ColorPicker;
use crate::util::SRGBA;
use anyhow::{anyhow, bail, Result};
use float_cmp::approx_eq;
use palette::{Clamp, IntoColor, Lch, RelativeContrast};

#[derive(Debug, Default, Copy, Clone)]
pub struct Exact();

impl ColorPicker for Exact {
    fn pick_color_graphic(
        &self,
        color: SRGBA,
        contrast: f32,
        grayscale: bool,
        lighten: Option<bool>,
    ) -> (SRGBA, Option<anyhow::Error>) {
        let mut err = None;

        let res = self.pick_color(color, Some(contrast), grayscale, lighten);
        if let Ok(c) = res {
            return (c, err);
        } else if let Err(e) = res {
            err = Some(anyhow!("Graphic contrast {} failed: {}", contrast, e));
        }

        let res = self.pick_color(color, None, grayscale, lighten);
        if let Ok(c) = res {
            return (c, err);
        } else if let Err(e) = res {
            err = Some(e);
        }

        (SRGBA::default(), err)
    }

    fn pick_color_text(
        &self,
        color: SRGBA,
        grayscale: bool,
        lighten: Option<bool>,
    ) -> (SRGBA, Option<anyhow::Error>) {
        let mut err = None;

        // AAA
        let res = self.pick_color(color, Some(7.0), grayscale, lighten);
        if let Ok(c) = res {
            return (c, err);
        } else if let Err(e) = res {
            err = Some(anyhow!("AAA text contrast failed: {}", e));
        }

        // AA
        let res = self.pick_color(color, Some(4.5), grayscale, lighten);
        if let Ok(c) = res {
            return (c, err);
        } else if let Err(e) = res {
            err = Some(anyhow!("AA text contrast failed: {}", e));
        }

        let res = self.pick_color(color, None, grayscale, lighten);
        if let Ok(c) = res {
            return (c, err);
        } else if let Err(e) = res {
            err = Some(e);
        }

        (SRGBA::default(), err)
    }

    fn pick_color(
        &self,
        color: SRGBA,
        contrast: Option<f32>,
        grayscale: bool,
        lighten: Option<bool>,
    ) -> Result<SRGBA> {
        let mut lch_color: Lch = (*color).into_color();

        // set to grayscale
        if grayscale {
            lch_color.chroma = 0.0;
        }

        // lighten or darken
        // TODO closed form solution using Lch color space contrast formula?
        // for now do binary search...

        if let Some(contrast) = contrast {
            let (min, max) = match lighten {
                Some(b) if b => (lch_color.l, 100.0),
                Some(_) => (0.0, lch_color.l),
                None => (0.0, 100.0),
            };
            let (mut l, mut r) = (min, max);

            for _ in 0..100 {
                dbg!((l, r));
                let cur_guess_lightness = (l + r) / 2.0;
                let mut cur_guess = lch_color;
                cur_guess.l = cur_guess_lightness;
                let cur_contrast = color.get_contrast_ratio(&cur_guess.into_color());
                let contrast_dir = contrast > cur_contrast;
                let lightness_dir = lch_color.l < cur_guess.l;
                if approx_eq!(f32, contrast, cur_contrast, ulps = 4) {
                    lch_color = cur_guess;
                    break;
                    // TODO fix
                } else if lightness_dir && contrast_dir || !lightness_dir && !contrast_dir {
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
        } else {
            // maximize contrast if no constraint is given
            if lch_color.l > 50.0 {
                Ok(SRGBA(palette::named::BLACK.into_format().into_color()))
            } else {
                Ok(SRGBA(palette::named::WHITE.into_format().into_color()))
            }
        }
    }
}
