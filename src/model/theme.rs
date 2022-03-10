// SPDX-License-Identifier: GPL-3.0-only

use super::{Selection, ThemeConstraints};
use crate::{
    color_picker::{ColorPicker, Exact},
    util::SRGBA,
};

#[derive(Copy, Clone, PartialEq, Debug, Default)]
pub struct Theme {
    // selected colors
    background: SRGBA,
    primary_container: SRGBA,
    secondary_container: SRGBA,
    accent: SRGBA,
    accent_text: SRGBA,
    accent_nav_handle_text: SRGBA,
    destructive: SRGBA,

    // derived surface colors
    window_header_background: SRGBA,
    background_component: SRGBA,
    background_component_divider: SRGBA,
    primary_component: SRGBA,
    primary_component_divider: SRGBA,
    secondary_component: SRGBA,
    secondary_component_divider: SRGBA,

    // derived text colors
    background_text: SRGBA,
    background_text_opacity80: SRGBA,
    primary_container_text: SRGBA,
    primary_container_text_opacity80: SRGBA,
    secondary_container_text: SRGBA,
    secondary_container_text_opacity80: SRGBA,
    background_component_text: SRGBA,
    background_component_text_opacity80: SRGBA,
    primary_container_component_text: SRGBA,
    primary_container_component_text_opacity80: SRGBA,
    secondary_container_component_text: SRGBA,
    secondary_container_component_text_opacity80: SRGBA,
    text_button_text: SRGBA,
    suggested_button_text: SRGBA,
    destructive_button_text: SRGBA,
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
        format!(
            r#"/* WIP CSS preview generation */
.background {{
background-color: {background}
}}

.background-component {{
background-color: {background_component}
}}

.background-componenet-divider {{
background-color: {background_component_divider}
}}

.primary-container {{
background-color: {primary_container}
}}

.primary-component {{
background-color: {primary_component}
}}

.primary-componenet-divider {{
background-color: {primary_component_divider}
}}

.secondary-container {{
background-color: {secondary_container}
}}

.secondary-component {{
background-color: {secondary_component}
}}

.secondary-componenet-divider {{
background-color: {secondary_component_divider}
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
