// SPDX-License-Identifier: GPL-3.0-only

use gtk4::gdk::RGBA;
use palette::rgb::Srgba;

pub fn srgba_from_rgba(rgba: RGBA) -> Srgba {
    Srgba::new(rgba.red(), rgba.blue(), rgba.green(), rgba.alpha())
}
