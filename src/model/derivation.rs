use super::{Selection, ThemeConstraints};
use crate::{
    color_picker::{ColorPicker, Exact},
    util::SRGBA,
};
#[derive(Copy, Clone, PartialEq, Debug, Default)]
pub struct ContainerDerivation {
    pub container: SRGBA,
    pub container_component: WidgetStates,
    pub container_divider: SRGBA,
    pub container_text: SRGBA,
    pub container_text_opacity_80: SRGBA,
}
// let accent_text = accent_text.unwrap_or(accent);
//  let accent_nav_handle_text = accent_nav_handle_text.unwrap_or(accent);
//  let picker = Exact::default();

//  let window_header_background = background;
//  let background_component =
//      picker.pick_color(background, elevated_contrast_ratio, false, Some(lighten))?;
//  let background_divider = picker.pick_color(
//      background,
//      divider_contrast_ratio,
//      divider_gray_scale,
//      Some(lighten),
//  )?;
//  let background_component_divider = picker.pick_color(
//      background_component,
//      divider_contrast_ratio,
//      divider_gray_scale,
//      Some(lighten),
//  )?;
//  let background_text =
//      picker.pick_color(background, text_contrast_ratio, true, Some(lighten))?;
//  let background_component_text = picker.pick_color(
//      background_component,
//      text_contrast_ratio,
//      true,
//      Some(lighten),
//  )?;

//  let primary_container_component = picker.pick_color(
//      primary_container,
//      elevated_contrast_ratio,
//      false,
//      Some(lighten),
//  )?;
//  let primary_container_component_divider = picker.pick_color(
//      primary_container,
//      divider_contrast_ratio,
//      divider_gray_scale,
//      Some(lighten),
//  )?;

//  let primary_container_component_text = picker.pick_color(
//      primary_container_component,
//      text_contrast_ratio,
//      true,
//      Some(lighten),
//  )?;
//  let primary_container_text =
//      picker.pick_color(primary_container, text_contrast_ratio, true, Some(lighten))?;

//  let secondary_container_component = picker.pick_color(
//      secondary_container,
//      elevated_contrast_ratio,
//      false,
//      Some(lighten),
//  )?;
//  let secondary_container_component_divider = picker.pick_color(
//      secondary_container,
//      divider_contrast_ratio,
//      divider_gray_scale,
//      Some(lighten),
//  )?;
//  let secondary_container_component_text = picker.pick_color(
//      secondary_container_component,
//      text_contrast_ratio,
//      true,
//      Some(lighten),
//  )?;
//  let secondary_container_text = picker.pick_color(
//      secondary_container,
//      text_contrast_ratio,
//      true,
//      Some(lighten),
//  )?;

//  let destructive_button_text =
//      picker.pick_color(destructive, text_contrast_ratio, true, None)?;

//  let suggested = accent;
//  let suggested_button_text =
//      picker.pick_color(suggested, text_contrast_ratio, true, None)?;

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Container {
    Background,
    Primary,
    Secondary,
}

impl TryFrom<(Selection, ThemeConstraints, Container)> for ContainerDerivation {
    type Error = anyhow::Error;

    fn try_from(
        (selection, constraints, Container): (Selection, ThemeConstraints, Container),
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
        todo!()
    }
}

#[derive(Copy, Clone, PartialEq, Debug, Default)]
pub struct AccentDerivation {
    pub accent: WidgetStates,
    pub accent_text: SRGBA,
    pub accent_nav_handle_text: SRGBA,
    pub suggested: WidgetStates,
}

impl TryFrom<(Selection, ThemeConstraints)> for AccentDerivation {
    type Error = anyhow::Error;

    fn try_from(
        (selection, constraints): (Selection, ThemeConstraints),
    ) -> Result<Self, Self::Error> {
        let Selection {
            background,
            primary_container,
            secondary_container,
            ..
        } = selection;

        let ThemeConstraints {
            elevated_contrast_ratio,
            divider_contrast_ratio,
            text_contrast_ratio,
            divider_gray_scale,
            lighten,
        } = constraints;
        todo!()
    }
}

#[derive(Copy, Clone, PartialEq, Debug, Default)]
pub struct DestructiveDerivation {
    pub destructive: WidgetStates,
}

impl TryFrom<(Selection, ThemeConstraints)> for DestructiveDerivation {
    type Error = anyhow::Error;

    fn try_from(
        (selection, constraints): (Selection, ThemeConstraints),
    ) -> Result<Self, Self::Error> {
        let Selection {
            accent,
            accent_text,
            accent_nav_handle_text,
            ..
        } = selection;

        let ThemeConstraints {
            elevated_contrast_ratio,
            divider_contrast_ratio,
            text_contrast_ratio,
            divider_gray_scale,
            lighten,
        } = constraints;
        todo!()
    }
}

#[derive(Copy, Clone, PartialEq, Debug, Default)]
pub struct WidgetStates {
    default: SRGBA,
    hover: SRGBA,
    pressed: SRGBA,
    focused: SRGBA,
    divider: SRGBA,
    text: SRGBA,
    // XXX this should ideally maintain AAA contrast, and failing that, color chooser should raise warnings
    text_opacity_80: SRGBA,
    // these are transparent but are not required to maintain contrast
    disabled: SRGBA,
    disabled_text: SRGBA,
}

impl TryFrom<(SRGBA, ThemeConstraints)> for WidgetStates {
    type Error = anyhow::Error;

    fn try_from((default, constraints): (SRGBA, ThemeConstraints)) -> Result<Self, Self::Error> {
        let ThemeConstraints {
            elevated_contrast_ratio,
            divider_contrast_ratio,
            text_contrast_ratio,
            divider_gray_scale,
            lighten,
        } = constraints;
        todo!()
    }
}
