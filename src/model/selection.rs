// SPDX-License-Identifier: GPL-3.0-only

use crate::util::srgba_from_rgba;
use gtk4::gdk::RGBA;
use palette::rgb::Srgba;

#[derive(Copy, Clone, Debug, Default)]
pub struct Selection {
    pub background: Srgba,
    pub primary_container: Srgba,
    pub secondary_container: Srgba,
    pub accent: Srgba,
    pub accent_text: Option<Srgba>,
    pub accent_nav_handle_text: Option<Srgba>,
    pub destructive: Srgba,
}

impl Selection {
    pub fn set_background(&mut self, rgba: RGBA) {
        self.background = srgba_from_rgba(rgba);
    }

    pub fn set_primary_container(&mut self, rgba: RGBA) {
        self.background = srgba_from_rgba(rgba);
    }

    pub fn set_secondary_container(&mut self, rgba: RGBA) {
        self.background = srgba_from_rgba(rgba);
    }

    pub fn set_accent_color(&mut self, rgba: RGBA) {
        self.background = srgba_from_rgba(rgba);
    }

    pub fn set_accent_text_color(&mut self, rgba: RGBA) {
        self.background = srgba_from_rgba(rgba);
    }

    pub fn set_accent_nav_handle_text_color(&mut self, rgba: RGBA) {
        self.background = srgba_from_rgba(rgba);
    }

    pub fn set_destructive(&mut self, rgba: RGBA) {
        self.background = srgba_from_rgba(rgba);
    }
}
