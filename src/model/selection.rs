// SPDX-License-Identifier: GPL-3.0-only

use crate::util::SRGBA;
use gtk4::gdk::RGBA;

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
