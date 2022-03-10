// SPDX-License-Identifier: GPL-3.0-only

use super::{Selection, ThemeConstraints};
use crate::color_picker::{ColorPicker, Exact};
use hex::encode;
use palette::{rgb::Srgba, Pixel};

#[derive(Copy, Clone, PartialEq, Debug, Default)]
pub struct Theme {
    // selected colors
    background: Srgba,
    primary_container: Srgba,
    secondary_container: Srgba,
    accent: Srgba,
    accent_text: Srgba,
    accent_nav_handle_text: Srgba,
    destructive: Srgba,

    // derived surface colors
    window_header_background: Srgba,
    background_component: Srgba,
    background_component_divider: Srgba,
    primary_component: Srgba,
    primary_component_divider: Srgba,
    secondary_component: Srgba,
    secondary_component_divider: Srgba,

    // derived text colors
    background_text: Srgba,
    background_text_opacity80: Srgba,
    primary_container_text: Srgba,
    primary_container_text_opacity80: Srgba,
    secondary_container_text: Srgba,
    secondary_container_text_opacity80: Srgba,
    background_component_text: Srgba,
    background_component_text_opacity80: Srgba,
    primary_container_component_text: Srgba,
    primary_container_component_text_opacity80: Srgba,
    secondary_container_component_text: Srgba,
    secondary_container_component_text_opacity80: Srgba,
    text_button_text: Srgba,
    suggested_button_text: Srgba,
    destructive_button_text: Srgba,
    // TODO
    // derived from button state
    // derived from selectable state
    // derived from draggable state
}

impl Theme {
    pub fn as_css(&self) -> String {
        let Self {
            background,
            primary_container,
            secondary_container,
            accent,
            accent_text,
            accent_nav_handle_text,
            destructive,
            window_header_background,
            background_component,
            background_component_divider,
            primary_component,
            primary_component_divider,
            secondary_component,
            secondary_component_divider,
            background_text,
            background_text_opacity80,
            primary_container_text,
            primary_container_text_opacity80,
            secondary_container_text,
            secondary_container_text_opacity80,
            background_component_text,
            background_component_text_opacity80,
            primary_container_component_text,
            primary_container_component_text_opacity80,
            secondary_container_component_text,
            secondary_container_component_text_opacity80,
            text_button_text,
            suggested_button_text,
            destructive_button_text,
        } = self;
        let background_hex = encode::<[u8; 4]>(Srgba::into_raw(background.into_format()));
        let background_component_hex =
            encode::<[u8; 4]>(Srgba::into_raw(background_component.into_format()));

        let primary_container_hex =
            encode::<[u8; 4]>(Srgba::into_raw(primary_container.into_format()));

        let secondary_container_hex =
            encode::<[u8; 4]>(Srgba::into_raw(secondary_container.into_format()));

        format!(
            r#"/* WIP CSS preview generation */
.background {{
background-color: #{background_hex}
}}

.background-component {{
background-color: #{background_component_hex}
}}


.primary-container {{
background-color: #{primary_container_hex}
}}

.secondary-container {{
background-color: #{secondary_container_hex}
}}
"#
        )
    }
}

impl TryFrom<(Selection, ThemeConstraints)> for Theme {
    type Error = anyhow::Error;

    fn try_from(
        (selection, constraints): (Selection, ThemeConstraints),
    ) -> Result<Self, Self::Error> {
        let Selection {
            background,
            primary_container,
            secondary_container,
            accent,
            accent_text,
            accent_nav_handle_text,
            destructive,
        } = selection;

        let ThemeConstraints {
            elevated_contrast_ratio,
            divider_contrast_ratio,
            text_contrast_ratio,
            divider_gray_scale,
            lighten,
        } = constraints;

        let accent_text = accent_text.unwrap_or(accent);
        let accent_nav_handle_text = accent_nav_handle_text.unwrap_or(accent);
        let picker = Exact::default();

        let window_header_background = background;
        let background_component =
            picker.pick_color(background, elevated_contrast_ratio, false, lighten)?;
        let background_component_divider = picker.pick_color(
            background,
            divider_contrast_ratio,
            divider_gray_scale,
            lighten,
        )?;
        let primary_component =
            picker.pick_color(primary_container, elevated_contrast_ratio, false, lighten)?;
        let primary_component_divider = picker.pick_color(
            primary_container,
            divider_contrast_ratio,
            divider_gray_scale,
            lighten,
        )?;
        let secondary_component =
            picker.pick_color(secondary_container, elevated_contrast_ratio, false, lighten)?;
        let secondary_component_divider = picker.pick_color(
            secondary_container,
            divider_contrast_ratio,
            divider_gray_scale,
            lighten,
        )?;
        // TODO allow input of color search heuristic to customize derivation results.
        // For now, simplest heuristic will be used which maintains hues, and only adjusts lightness to get the exact minumum contrast
        //

        Ok(Self {
            background,
            primary_container,
            secondary_container,
            accent,
            accent_text,
            accent_nav_handle_text,
            destructive,
            window_header_background,
            background_component,
            background_component_divider,
            primary_component,
            primary_component_divider,
            secondary_component,
            secondary_component_divider,
            ..Default::default()
        })
    }
}
