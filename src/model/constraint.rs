// SPDX-License-Identifier: GPL-3.0-only

#[derive(Copy, Clone, Debug)]
pub struct ThemeConstraints {
    pub elevated_contrast_ratio: f32,
    pub divider_contrast_ratio: f32,
    pub text_contrast_ratio: f32,
    pub divider_gray_scale: bool,
    pub lighten: bool,
}

impl Default for ThemeConstraints {
    fn default() -> Self {
        Self {
            elevated_contrast_ratio: 1.1,
            divider_contrast_ratio: 1.51,
            text_contrast_ratio: 7.0,
            divider_gray_scale: true,
            lighten: true,
        }
    }
}
