use super::{Selection, ThemeConstraints};
use palette::rgb::Srgb;

#[derive(Copy, Clone, PartialEq, Debug, Default)]
pub struct Theme {
    // selected colors
    background: Srgb,
    primary_container: Srgb,
    secondary_container: Srgb,
    accent_color: Srgb,
    accent_text_color: Srgb,
    accent_nav_handle_text_color: Srgb,
    destructive_color: Srgb,

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
    type Error = &'static str;

    fn try_from(value: (Selection, ThemeConstraints)) -> Result<Self, Self::Error> {
        todo!()
    }
}
