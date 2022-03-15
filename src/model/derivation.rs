use super::{Selection, ThemeConstraints};
use crate::{color_picker::ColorPicker, util::SRGBA};
use anyhow::bail;
use palette::{IntoColor, Lcha, Shade, Srgba};
use std::fmt;

#[derive(Copy, Clone, PartialEq, Debug, Default)]
pub struct ContainerDerivation {
    pub prefix: Container,
    pub container: SRGBA,
    pub container_component: Widget,
    pub container_divider: SRGBA,
    pub container_text: SRGBA,
    pub container_text_opacity_80: SRGBA,
}

pub trait AsCss {
    fn as_css(&self) -> String;
}

impl AsCss for ContainerDerivation {
    fn as_css(&self) -> String {
        let Self {
            prefix,
            container,
            container_component,
            container_divider,
            container_text,
            container_text_opacity_80,
        } = self;
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
        } = container_component;

        let prefix_lower = match self.prefix {
            Container::Background => "background",
            Container::Primary => "primary-container",
            Container::Secondary => "secondary-container",
        };
        format!(
            r#"/* {prefix_lower} CSS */
*.{prefix_lower} {{
  background-color: {container};
  color: {container_text};
  border-radius: 8px;
}}

*.{prefix_lower}-component {{
  background-color: {default};
  color: {text};
  border-radius: 8px;
}}

*.{prefix_lower}-component:hover {{
  background-color: {hover};
  color: {text};
  border-radius: 8px;
}}

*.{prefix_lower}-component:focus {{
  background-color: {focused};
  outline-color: {default};
  color: {text};
  border-radius: 8px;
}}

*.{prefix_lower}-component:active {{
  background-color: {pressed};
  color: {text};
  border-radius: 8px;
}}

*.{prefix_lower}-component:disabled {{
  background-color: {disabled};
  color: {text};
  border-radius: 8px;
}}

*.{prefix_lower}-divider {{
  background-color: {container_divider};
}}

*.{prefix_lower}-divider {{
  background-color: {divider};
}}
"#
        )
    }
}

// TODO use for descriptive error messages below...
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Container {
    Background,
    Primary,
    Secondary,
}

impl Default for Container {
    fn default() -> Self {
        Self::Background
    }
}

impl fmt::Display for Container {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Container::Background => write!(f, "Background"),
            Container::Primary => write!(f, "Primary Container"),
            Container::Secondary => write!(f, "Secondary Container"),
        }
    }
}

impl<T: ColorPicker> TryFrom<(Selection, ThemeConstraints, T, Container)> for ContainerDerivation {
    type Error = anyhow::Error;

    fn try_from(
        (selection, constraints, picker, container_type): (
            Selection,
            ThemeConstraints,
            T,
            Container,
        ),
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

        let container = match container_type {
            Container::Background => background,
            Container::Primary => primary_container,
            Container::Secondary => secondary_container,
        };
        let container_divider = match picker.pick_color(
            container,
            divider_contrast_ratio,
            divider_gray_scale,
            Some(lighten),
        ) {
            Ok(c) => c,
            Err(e) => bail!("{} => \"container divider\" failed: {}", container_type, e),
        };

        let container_text = match picker.pick_color(container, text_contrast_ratio, true, None) {
            Ok(c) => c,
            Err(e) => bail!("{} => \"container text\" failed: {}", container_type, e),
        };

        // TODO revisit this and adjust constraints for transparency
        let mut container_text_opacity_80 = container_text;
        (*container_text_opacity_80).alpha *= 0.8;

        let component_default =
            match picker.pick_color(container, elevated_contrast_ratio, false, Some(lighten)) {
                Ok(c) => c,
                Err(e) => bail!(
                    "{} => \"container component default\" failed: {}",
                    container_type,
                    e
                ),
            };

        let container_component = match Widget::try_from((component_default, constraints, picker)) {
            Ok(c) => c,
            Err(e) => bail!(
                "{} => \"container component derivation\" failed: {}",
                container_type,
                e
            ),
        };

        Ok(Self {
            prefix: container_type,
            container,
            container_divider,
            container_text,
            container_text_opacity_80,
            container_component,
        })
    }
}

#[derive(Copy, Clone, PartialEq, Debug, Default)]
pub struct AccentDerivation {
    pub accent: SRGBA,
    pub accent_text: SRGBA,
    pub accent_nav_handle_text: SRGBA,
    pub suggested: Widget,
}

impl<T: ColorPicker> TryFrom<(Selection, ThemeConstraints, T)> for AccentDerivation {
    type Error = anyhow::Error;

    fn try_from(
        (selection, constraints, picker): (Selection, ThemeConstraints, T),
    ) -> Result<Self, Self::Error> {
        let Selection {
            accent,
            accent_text,
            accent_nav_handle_text,
            ..
        } = selection;

        let suggested = Widget::try_from((accent, constraints, picker))?;
        let accent_text = accent_text.unwrap_or(accent);
        let accent_nav_handle_text = accent_nav_handle_text.unwrap_or(accent);

        Ok(Self {
            accent,
            accent_text,
            accent_nav_handle_text,
            suggested,
        })
    }
}

impl AsCss for AccentDerivation {
    fn as_css(&self) -> String {
        let AccentDerivation {
            accent,
            accent_text,
            accent_nav_handle_text,
            suggested,
        } = self;

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

        format!(
            r#"/* Suggested CSS */
*.suggested-action {{
  background-color: {default};
  color: {text};
  border-radius: 8px;
}}

*.suggested-action:hover {{
  background-color: {hover};
  color: {text};
  border-radius: 8px;
}}

*.suggested-action:focus {{
  background-color: {focused};
  outline-color: {default};
  color: {text};
  border-radius: 8px;
}}

*.suggested-action:active {{
  background-color: {pressed};
  color: {text};
  border-radius: 8px;
}}

*.suggested-action:disabled {{
  background-color: {disabled};
  color: {text};
  border-radius: 8px;
}}

"#
        )
    }
}

#[derive(Copy, Clone, PartialEq, Debug, Default)]
pub struct DestructiveDerivation {
    pub destructive: Widget,
}

impl<T: ColorPicker> TryFrom<(Selection, ThemeConstraints, T)> for DestructiveDerivation {
    type Error = anyhow::Error;

    fn try_from(
        (selection, constraints, picker): (Selection, ThemeConstraints, T),
    ) -> Result<Self, Self::Error> {
        Ok(Self {
            destructive: (selection.destructive, constraints, picker).try_into()?,
        })
    }
}

impl AsCss for DestructiveDerivation {
    fn as_css(&self) -> String {
        let DestructiveDerivation { destructive } = &self;
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

        format!(
            r#"/* Destructive CSS */
*.destructive-action {{
  background-color: {default};
  color: {text};
  border-radius: 8px;
}}

*.destructive-action:hover {{
  background-color: {hover};
  color: {text};
  border-radius: 8px;
}}

*.destructive-action:focus {{
  background-color: {focused};
  outline-color: {default};
  color: {text};
  border-radius: 8px;
}}

*.destructive-action:active {{
  background-color: {pressed};
  color: {text};
  border-radius: 8px;
}}

*.destructive-action:disabled {{
  background-color: {disabled};
  color: {text};
  border-radius: 8px;
}}

"#
        )
    }
}

#[derive(Copy, Clone, PartialEq, Debug, Default)]
pub struct Widget {
    pub default: SRGBA,
    pub hover: SRGBA,
    pub pressed: SRGBA,
    pub focused: SRGBA,
    pub divider: SRGBA,
    pub text: SRGBA,
    // XXX this should ideally maintain AAA contrast, and failing that, color chooser should raise warnings
    pub text_opacity_80: SRGBA,
    // these are transparent but are not required to maintain contrast
    pub disabled: SRGBA,
    pub disabled_text: SRGBA,
}

impl<T: ColorPicker> TryFrom<(SRGBA, ThemeConstraints, T)> for Widget {
    type Error = anyhow::Error;

    fn try_from(
        (default, constraints, picker): (SRGBA, ThemeConstraints, T),
    ) -> Result<Self, Self::Error> {
        let ThemeConstraints {
            divider_contrast_ratio,
            text_contrast_ratio,
            divider_gray_scale,
            lighten,
            ..
        } = constraints;

        let lch = Lcha {
            color: default.color.into_color(),
            alpha: default.alpha,
        };

        let hover = if lighten {
            lch.lighten(0.1)
        } else {
            lch.darken(0.1)
        };

        let pressed = if lighten {
            hover.lighten(0.1)
        } else {
            hover.darken(0.1)
        };
        let pressed = SRGBA(Srgba {
            color: pressed.color.into_color(),
            alpha: pressed.alpha,
        });

        // TODO is this actually a different color? or just outlined?
        let focused = default;

        let mut disabled = default;
        (*disabled).alpha = 0.5;

        let divider = picker.pick_color(
            pressed,
            divider_contrast_ratio,
            divider_gray_scale,
            Some(lighten),
        )?;
        let text = picker.pick_color(pressed, text_contrast_ratio, true, None)?;
        let mut text_opacity_80 = text;
        (*text_opacity_80).alpha = 0.8;

        let mut disabled_text = text;
        (*disabled_text).alpha = 0.5;

        Ok(Self {
            default,
            hover: SRGBA(Srgba {
                color: hover.color.into_color(),
                alpha: hover.alpha,
            }),
            pressed,
            focused,
            divider,
            text,
            text_opacity_80,
            disabled,
            disabled_text,
        })
    }
}
