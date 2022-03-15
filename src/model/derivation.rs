use super::{Selection, ThemeConstraints};
use crate::{color_picker::ColorPicker, util::SRGBA};
use anyhow::anyhow;
use palette::{IntoColor, Lcha, Shade, Srgba};
use std::fmt;

#[derive(Copy, Clone, PartialEq, Debug, Default)]
pub struct Container {
    pub prefix: ContainerType,
    pub container: SRGBA,
    pub container_component: Widget,
    pub container_divider: SRGBA,
    pub container_text: SRGBA,
    pub container_text_opacity_80: SRGBA,
}

// TODO special styling for switches in gtk4
pub trait AsCss {
    fn as_css(&self) -> String;
}

impl AsCss for Container {
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
            ContainerType::Background => "background",
            ContainerType::Primary => "primary-container",
            ContainerType::Secondary => "secondary-container",
        };
        let top_level_border_radius = if self.prefix != ContainerType::Background {
            "border-radius: 8px;"
        } else {
            ""
        };

        format!(
            r#"/* {prefix_lower} CSS */
*.{prefix_lower} {{
  background-color: {container};
  color: {container_text};
  {top_level_border_radius}
}}

*.{prefix_lower}-component {{
  background-color: {default};
  color: {text};
  border-radius: 8px;
  border-color: {default};
}}

*.{prefix_lower}-component:hover {{
  background-color: {hover};
  color: {text};
  border-radius: 8px;
  border-color: {default};
}}

*.{prefix_lower}-component:selected {{
  background-color: {focused};
  outline-color: {default};
  color: {text};
  border-radius: 8px;
  border-color: {default};
}}

*.{prefix_lower}-component:active {{
  background-color: {pressed};
  color: {text};
  border-radius: 8px;
  border-color: {default};
}}

*.{prefix_lower}-component:disabled {{
  background-color: {disabled};
  color: {text};
  border-radius: 8px;
  border-color: {default};
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

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum ContainerType {
    Background,
    Primary,
    Secondary,
}

impl Default for ContainerType {
    fn default() -> Self {
        Self::Background
    }
}

impl fmt::Display for ContainerType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ContainerType::Background => write!(f, "Background"),
            ContainerType::Primary => write!(f, "Primary Container"),
            ContainerType::Secondary => write!(f, "Secondary Container"),
        }
    }
}

#[derive(Debug)]
pub struct ContainerDerivation {
    pub container: Container,
    pub errors: Vec<anyhow::Error>,
}

impl<T: ColorPicker> From<(Selection, ThemeConstraints, T, ContainerType)> for ContainerDerivation {
    fn from(
        (selection, constraints, picker, container_type): (
            Selection,
            ThemeConstraints,
            T,
            ContainerType,
        ),
    ) -> Self {
        let mut errors = Vec::new();

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
            ContainerType::Background => background,
            ContainerType::Primary => primary_container,
            ContainerType::Secondary => secondary_container,
        };
        let (container_divider, err) = picker.pick_color_graphic(
            container,
            divider_contrast_ratio,
            divider_gray_scale,
            Some(lighten),
        );
        if let Some(e) = err {
            errors.push(e);
        };

        let (container_text, err) = picker.pick_color_text(container, true, None);
        if let Some(err) = err {
            let err = anyhow!("{} => \"container text\" failed: {}", container_type, err);
            errors.push(err);
        };

        // TODO revisit this and adjust constraints for transparency
        let mut container_text_opacity_80 = container_text;
        (*container_text_opacity_80).alpha *= 0.8;

        let (component_default, err) =
            picker.pick_color_graphic(container, elevated_contrast_ratio, false, Some(lighten));
        if let Some(e) = err {
            let err = anyhow!(
                "{} => \"container component\" failed: {}",
                container_type,
                e
            );
            errors.push(err);
        };

        let WidgetDerivation {
            widget: container_component,
            errors: errs,
        } = (component_default, constraints, picker).into();
        for e in errs {
            let err = anyhow!(
                "{} => \"container component derivation\" failed: {}",
                container_type,
                e
            );
            errors.push(err);
        }

        Self {
            container: Container {
                prefix: container_type,
                container,
                container_divider,
                container_text,
                container_text_opacity_80,
                container_component,
            },
            errors,
        }
    }
}

#[derive(Copy, Clone, PartialEq, Debug, Default)]
pub struct Accent {
    pub accent: SRGBA,
    pub accent_text: SRGBA,
    pub accent_nav_handle_text: SRGBA,
    pub suggested: Widget,
}

#[derive(Debug)]
pub struct AccentDerivation {
    pub accent: Accent,
    pub errors: Vec<anyhow::Error>,
}

impl<T: ColorPicker> From<(Selection, ThemeConstraints, T)> for AccentDerivation {
    fn from((selection, constraints, picker): (Selection, ThemeConstraints, T)) -> Self {
        let Selection {
            accent,
            accent_text,
            accent_nav_handle_text,
            ..
        } = selection;
        let mut errors = Vec::<anyhow::Error>::new();

        let WidgetDerivation {
            widget: suggested,
            errors: errs,
        } = (accent, constraints, picker).into();
        for e in errs {
            errors.push(anyhow!("\"Accent component derivation\" failed: {}", e));
        }
        let accent_text = accent_text.unwrap_or(accent);
        let accent_nav_handle_text = accent_nav_handle_text.unwrap_or(accent);

        Self {
            accent: Accent {
                accent,
                accent_text,
                accent_nav_handle_text,
                suggested,
            },
            errors,
        }
    }
}

impl AsCss for Accent {
    fn as_css(&self) -> String {
        let Accent {
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
  border-color: {default};
}}

*.suggested-action:hover {{
  background-color: {hover};
  color: {text};
  border-radius: 8px;
  border-color: {default};
}}

*.suggested-action:selected {{
  background-color: {focused};
  outline-color: {default};
  color: {text};
  border-radius: 8px;
  border-color: {default};
}}

*.suggested-action:active {{
  background-color: {pressed};
  color: {text};
  border-radius: 8px;
  border-color: {default};
}}

*.suggested-action:disabled {{
  background-color: {disabled};
  color: {text};
  border-radius: 8px;
  border-color: {default};
}}

"#
        )
    }
}

#[derive(Copy, Clone, PartialEq, Debug, Default)]
pub struct Destructive {
    pub destructive: Widget,
}

#[derive(Debug)]
pub struct DestructiveDerivation {
    pub destructive: Destructive,
    pub errors: Vec<anyhow::Error>,
}

impl<T: ColorPicker> From<(Selection, ThemeConstraints, T)> for DestructiveDerivation {
    fn from((selection, constraints, picker): (Selection, ThemeConstraints, T)) -> Self {
        let mut errors = Vec::<anyhow::Error>::new();

        let WidgetDerivation {
            widget: destructive,
            errors: errs,
        } = (selection.destructive, constraints, picker).into();
        for e in errs {
            errors.push(anyhow!(
                "\"Destructive component derivation\" failed: {}",
                e
            ));
        }
        Self {
            destructive: Destructive { destructive },
            errors,
        }
    }
}

impl AsCss for Destructive {
    fn as_css(&self) -> String {
        let Destructive { destructive } = &self;
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
  border-color: {default};
}}

*.destructive-action:hover {{
  background-color: {hover};
  color: {text};
  border-radius: 8px;
  border-color: {default};
}}

*.destructive-action:seleceted {{
  background-color: {focused};
  outline-color: {default};
  color: {text};
  border-radius: 8px;
  border-color: {default};
}}

*.destructive-action:active {{
  background-color: {pressed};
  color: {text};
  border-radius: 8px;
  border-color: {default};
}}

*.destructive-action:disabled {{
  background-color: {disabled};
  color: {text};
  border-radius: 8px;
  border-color: {default};
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

pub struct WidgetDerivation {
    pub widget: Widget,
    pub errors: Vec<anyhow::Error>,
}

impl<T: ColorPicker> From<(SRGBA, ThemeConstraints, T)> for WidgetDerivation {
    fn from((default, constraints, picker): (SRGBA, ThemeConstraints, T)) -> Self {
        let ThemeConstraints {
            divider_contrast_ratio,
            text_contrast_ratio,
            divider_gray_scale,
            lighten,
            ..
        } = constraints;

        let mut errors = Vec::new();

        let lch = Lcha {
            color: default.color.into_color(),
            alpha: default.alpha,
        };

        // TODO define constraints for different states...
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

        let (divider, error) = picker.pick_color_graphic(
            pressed,
            divider_contrast_ratio,
            divider_gray_scale,
            Some(lighten),
        );
        if let Some(error) = error {
            errors.push(error);
        }

        let (text, error) = picker.pick_color_text(pressed, true, None);
        if let Some(error) = error {
            errors.push(error);
        }

        let mut text_opacity_80 = text;
        (*text_opacity_80).alpha = 0.8;

        let mut disabled_text = text;
        (*disabled_text).alpha = 0.5;

        Self {
            widget: Widget {
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
            },
            errors,
        }
    }
}
