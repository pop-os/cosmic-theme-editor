// SPDX-License-Identifier: GPL-3.0-only

use core::fmt;
use std::ops::{Deref, DerefMut};

use gtk4::{
    gdk::RGBA,
    gdk_pixbuf::{Colorspace, Pixbuf},
    gio::File,
    prelude::*,
};
use hex::encode;
use palette::{rgb::Srgba, Pixel};

pub fn hex_from_rgba(rgba: &Srgba) -> String {
    let hex = encode::<[u8; 4]>(Srgba::into_raw(rgba.into_format()));
    format!("#{hex}")
}
use kmeans_colors::{get_kmeans_hamerly, Kmeans, Sort};
use palette::{IntoColor, Lab, Srgb};

#[derive(Copy, Clone, PartialEq, Debug, Default)]
pub struct SRGBA(pub Srgba);

impl SRGBA {
    pub fn into_inner(self) -> Srgba {
        self.0
    }
}

impl From<RGBA> for SRGBA {
    fn from(rgba: RGBA) -> Self {
        Self(Srgba::new(
            rgba.red(),
            rgba.green(),
            rgba.blue(),
            rgba.alpha(),
        ))
    }
}

impl Into<RGBA> for SRGBA {
    fn into(self) -> RGBA {
        RGBA::new(self.red, self.green, self.blue, self.alpha)
    }
}

impl Deref for SRGBA {
    type Target = Srgba;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for SRGBA {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl fmt::Display for SRGBA {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", hex_from_rgba(&*self))
    }
}

pub fn palette_from_image(f: File) -> Option<Vec<SRGBA>> {
    // calculate kmeans colors from file
    if let Some(Ok(img)) = f.path().map(|p| Pixbuf::from_file(p)) {
        if img.bits_per_sample() == 8 && img.colorspace() == Colorspace::Rgb {
            let pixels = unsafe { img.pixels() };
            let lab: Vec<Lab> = if img.has_alpha() {
                Srgba::from_raw_slice(pixels)
                    .iter()
                    .map(|x| x.color.into_format().into_color())
                    .collect()
            } else {
                Srgb::from_raw_slice(pixels)
                    .iter()
                    .map(|x| x.into_format().into_color())
                    .collect()
            };

            let mut result = Kmeans::new();

            // TODO random seed
            for i in 0..2 {
                let run_result = get_kmeans_hamerly(5, 20, 5.0, false, &lab, i as u64);
                if run_result.score < result.score {
                    result = run_result;
                }
            }
            let mut res = Lab::sort_indexed_colors(&result.centroids, &result.indices);
            res.sort_unstable_by(|a, b| (b.percentage).partial_cmp(&a.percentage).unwrap());
            let colors: Vec<SRGBA> = res.iter().map(|x| SRGBA(x.centroid.into_color())).collect();
            Some(colors)
        } else {
            eprintln!("unsupported color format");
            // TODO error dialog msg
            None
        }
    } else {
        None
    }
}
