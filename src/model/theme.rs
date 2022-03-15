// SPDX-License-Identifier: GPL-3.0-only

use super::{
    Accent, AccentDerivation, AsCss, Container, ContainerDerivation, ContainerType, Destructive,
    DestructiveDerivation, Selection, ThemeConstraints,
};
use crate::{
    color_picker::{ColorPicker, Exact},
    util::SRGBA,
};

#[derive(Copy, Clone, PartialEq, Debug, Default)]
pub struct Theme {
    background: Container,
    primary: Container,
    secondary: Container,
    accent: Accent,
    destructive: Destructive,

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
        css.push_str(&accent.as_css());
        css.push_str(&destructive.as_css());
        let accent = accent.accent;
        css.push_str(&format!(
            r#"/* Accent CSS */
* {{
  background-image: none;
  outline-color: {accent};
}}

"#
        ));
        css
    }
}

pub struct ThemeDerivation {
    pub theme: Theme,
    pub errors: Vec<anyhow::Error>,
}

impl From<(Selection, ThemeConstraints)> for ThemeDerivation {
    fn from((selection, constraints): (Selection, ThemeConstraints)) -> Self {
        let picker = Exact::default();
        let mut theme_errors = Vec::new();
        let window_header_background = selection.background;
        let (text_button_text, err) = picker.pick_color_text(selection.background, true, None);
        if let Some(err) = err {
            theme_errors.push(err)
        };
        let ContainerDerivation {
            container: background,
            errors: mut errs,
        } = (selection, constraints, picker, ContainerType::Background).into();
        theme_errors.append(&mut errs);

        let ContainerDerivation {
            container: primary,
            errors: mut errs,
        } = (selection, constraints, picker, ContainerType::Primary).into();
        theme_errors.append(&mut errs);

        let ContainerDerivation {
            container: secondary,
            mut errors,
        } = (selection, constraints, picker, ContainerType::Secondary).into();
        theme_errors.append(&mut errors);

        let AccentDerivation { accent, mut errors } = (selection, constraints, picker).into();
        theme_errors.append(&mut errors);

        let DestructiveDerivation {
            destructive,
            mut errors,
        } = (selection, constraints, picker).into();
        theme_errors.append(&mut errors);

        Self {
            theme: Theme {
                background,
                primary,
                secondary,
                accent,
                destructive,
                window_header_background,
                text_button_text,
            },
            errors: theme_errors,
        }
    }
}
