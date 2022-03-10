// SPDX-License-Identifier: GPL-3.0-only

use crate::util::srgb_from_rgba;
use gtk4::gdk::RGBA;
use palette::rgb::Srgb;

#[derive(Copy, Clone, Debug, Default)]
pub struct Selection {
    pub background: Srgb,
    pub primary_container: Srgb,
    pub secondary_container: Srgb,
    pub accent: Srgb,
    pub accent_text: Option<Srgb>,
    pub accent_nav_handle_text: Option<Srgb>,
    pub destructive: Srgb,
}

impl Selection {
    pub fn set_background(&mut self, rgba: RGBA) {
        self.background = srgb_from_rgba(rgba);
    }

    pub fn set_primary_container(&mut self, rgba: RGBA) {
        self.background = srgb_from_rgba(rgba);
    }

    pub fn set_secondary_container(&mut self, rgba: RGBA) {
        self.background = srgb_from_rgba(rgba);
    }

    pub fn set_accent_color(&mut self, rgba: RGBA) {
        self.background = srgb_from_rgba(rgba);
    }

    pub fn set_accent_text_color(&mut self, rgba: RGBA) {
        self.background = srgb_from_rgba(rgba);
    }

    pub fn set_accent_nav_handle_text_color(&mut self, rgba: RGBA) {
        self.background = srgb_from_rgba(rgba);
    }

    pub fn set_destructive(&mut self, rgba: RGBA) {
        self.background = srgb_from_rgba(rgba);
    }
}
