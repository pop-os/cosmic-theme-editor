// SPDX-License-Identifier: GPL-3.0-only

use gtk4::gdk::RGBA;
use palette::{rgb::Srgb, IntoColor, Lab};

pub fn srgb_from_rgba(rgba: RGBA) -> Srgb {
    Srgb::new(rgba.red(), rgba.blue(), rgba.green())
}
