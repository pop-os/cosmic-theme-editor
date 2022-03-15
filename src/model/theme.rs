// SPDX-License-Identifier: GPL-3.0-only

use super::{
    AccentDerivation, AsCss, ContainerDerivation, DestructiveDerivation, Selection,
    ThemeConstraints,
};
use crate::{
    color_picker::{ColorPicker, Exact},
    model::{Container, Widget},
    util::SRGBA,
};

#[derive(Copy, Clone, PartialEq, Debug, Default)]
pub struct Theme {
    background: ContainerDerivation,
    primary: ContainerDerivation,
    secondary: ContainerDerivation,
    accent: AccentDerivation,
    destructive: DestructiveDerivation,

    // TODO derived surface colors which don't fit neatly in a category
    window_header_background: SRGBA,
    text_button_text: SRGBA,
}

impl Theme {
    pub fn as_css(&self) -> String {
        let Self {
            background,
            primary,
            secondary,
            accent,
            destructive,
            ..
        } = self;
        let mut css = String::new();

        css.push_str(&background.as_css());
        css.push_str(&primary.as_css());
        css.push_str(&secondary.as_css());

        {
            let AccentDerivation {
                accent,
                accent_text,
                accent_nav_handle_text,
                suggested,
            } = accent;

            let Widget {
                default,
                hover,
                pressed,
                focused,
                divider,
                text,
                // XXX this should ideally maintain AAA contrast, and failing that, color chooser should raise warnings
                text_opacity_80,
                // these are transparent but are not required to maintain contrast
                disabled,
                disabled_text,
            } = suggested;

            css.push_str(&format!(
                r#"/* Accent CSS */
* {{
  background-image: none;
  outline-color: {accent};
}}

button.suggested-action {{
  background-color: {default};
  color: {text};
}}
"#
            ))
        }

        {
            let DestructiveDerivation { destructive } = destructive;
            let Widget {
                default,
                hover,
                pressed,
                focused,
                divider,
                text,
                text_opacity_80,
                disabled,
                disabled_text,
            } = destructive;

            css.push_str(&format!(
                r#"
button.destructive-action {{
  background-color: {default};
  outline-color: {default};
  color: {text};
}}
"#
            ));
        }

        css
    }
}

impl TryFrom<(Selection, ThemeConstraints)> for Theme {
    type Error = anyhow::Error;

    fn try_from(
        (selection, constraints): (Selection, ThemeConstraints),
    ) -> Result<Self, Self::Error> {
        let picker = Exact::default();
        let window_header_background = selection.background;
        let text_button_text = match picker.pick_color(
            selection.background,
            constraints.text_contrast_ratio,
            true,
            None,
        ) {
            Ok(c) => c,
            Err(e) => {
                anyhow::bail!(
                    "Failed to derive text button text color from the selected background color."
                )
            }
        };

        Ok(Self {
            background: (selection, constraints, picker, Container::Background).try_into()?,
            primary: (selection, constraints, picker, Container::Primary).try_into()?,
            secondary: (selection, constraints, picker, Container::Secondary).try_into()?,
            accent: (selection, constraints, picker).try_into()?,
            destructive: (selection, constraints, picker).try_into()?,
            window_header_background,
            text_button_text,
        })
    }
}
