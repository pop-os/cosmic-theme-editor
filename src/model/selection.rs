// SPDX-License-Identifier: GPL-3.0-only

use crate::util::SRGBA;
use gtk4::gdk::RGBA;
use palette::{named, IntoColor, Lch};
use std::convert::TryFrom;

#[derive(Copy, Clone, Debug, Default)]
pub struct Selection {
    pub background: SRGBA,
    pub primary_container: SRGBA,
    pub secondary_container: SRGBA,
    pub accent: SRGBA,
    pub accent_text: Option<SRGBA>,
    pub accent_nav_handle_text: Option<SRGBA>,
    pub destructive: SRGBA,
}

impl Selection {
    pub fn set_background(&mut self, rgba: RGBA) {
        self.background = rgba.into();
    }

    pub fn set_primary_container(&mut self, rgba: RGBA) {
        self.primary_container = rgba.into();
    }

    pub fn set_secondary_container(&mut self, rgba: RGBA) {
        self.secondary_container = rgba.into();
    }

    pub fn set_accent_color(&mut self, rgba: RGBA) {
        self.accent = rgba.into();
    }

    pub fn set_accent_text_color(&mut self, rgba: RGBA) {
        self.accent_text = Some(rgba.into());
    }

    pub fn set_accent_nav_handle_text_color(&mut self, rgba: RGBA) {
        self.accent_nav_handle_text = Some(rgba.into());
    }

    pub fn set_destructive(&mut self, rgba: RGBA) {
        self.destructive = rgba.into();
    }
}

// vector should be in order of most common
impl TryFrom<Vec<SRGBA>> for Selection {
    type Error = anyhow::Error;

    fn try_from(mut colors: Vec<SRGBA>) -> Result<Self, Self::Error> {
        if colors.len() < 5 {
            anyhow::bail!("length of inputted vector must be at least 5.")
        } else {
            let lch_colors: Vec<Lch> = colors
                .iter()
                .map(|x| x.color.into_format().into_color())
                .collect();

            let red_lch: Lch = named::CRIMSON.into_format().into_color();
            let mut reddest_i = 1;
            for (i, c) in lch_colors[1..].iter().enumerate() {
                let d_cur = (c.hue.to_degrees() - red_lch.hue.to_degrees()).abs();
                let reddest_d =
                    (lch_colors[reddest_i].hue.to_degrees() - red_lch.hue.to_degrees()).abs();
                if d_cur < reddest_d {
                    reddest_i = i;
                }
            }

            let red = colors.remove(reddest_i);

            Ok(Self {
                background: colors[0],
                primary_container: colors[1],
                secondary_container: colors[3],
                accent: colors[2],
                accent_text: Some(colors[2]),
                accent_nav_handle_text: Some(colors[2]),
                destructive: red,
            })
        }
    }
}
