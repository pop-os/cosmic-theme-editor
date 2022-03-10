// SPDX-License-Identifier: GPL-3.0-only

use super::{Selection, ThemeConstraints};
use crate::color_picker::{ColorPicker, Exact};
use palette::rgb::Srgb;

#[derive(Copy, Clone, PartialEq, Debug, Default)]
pub struct Theme {
    // selected colors
    background: Srgb,
    primary_container: Srgb,
    secondary_container: Srgb,
    accent: Srgb,
    accent_text: Srgb,
    accent_nav_handle_text: Srgb,
    destructive: Srgb,

    // derived surface colors
    window_header_background: Srgb,
    background_component: Srgb,
    background_component_divider: Srgb,
    primary_component: Srgb,
    primary_component_divider: Srgb,
    secondary_component: Srgb,
    secondary_component_divider: Srgb,

    // derived text colors
    background_text: Srgb,
    background_text_opacity80: Srgb,
    primary_container_text: Srgb,
    primary_container_text_opacity80: Srgb,
    secondary_container_text: Srgb,
    secondary_container_text_opacity80: Srgb,
    background_component_text: Srgb,
    background_component_text_opacity80: Srgb,
    primary_container_component_text: Srgb,
    primary_container_component_text_opacity80: Srgb,
    secondary_container_component_text: Srgb,
    secondary_container_component_text_opacity80: Srgb,
    text_button_text: Srgb,
    suggested_button_text: Srgb,
    destructive_button_text: Srgb,
    // TODO
    // derived from button state
    // derived from selectable state
    // derived from draggable state
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
