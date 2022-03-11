// SPDX-License-Identifier: GPL-3.0-only

use super::{
    AccentDerivation, ContainerDerivation, DestructiveDerivation, Selection, ThemeConstraints,
};
use crate::util::SRGBA;

#[derive(Copy, Clone, PartialEq, Debug, Default)]
pub struct Theme {
    background: ContainerDerivation,
    accent: AccentDerivation,
    destructive: DestructiveDerivation,

    // TODO derived surface colors which don't fit neatly in a category
    window_header_background: SRGBA,
    text_button_text: SRGBA,
}

impl Theme {
    pub fn as_css(&self) -> String {
        todo!();
        //     let Self {
        //         } = self;
        //         format!(
        //             r#"/* WIP CSS preview generation */
        // .background {{
        // background-color: {background};
        // color: {background_text};
        // }}

        // .background-component {{
        // background-color: {background_component};
        // color: {background_component_text};
        // }}

        // .background-componenet-divider {{
        // background-color: {background_component_divider};
        // }}

        // .primary-container {{
        // background-color: {primary_container};
        // color: {primary_container_text};
        // }}

        // .primary-container-component {{
        // background-color: {primary_container_component};
        // color: {primary_container_component_text};
        // }}

        // .primary-container-componenet-divider {{
        // background-color: {primary_container_component_divider};
        // }}

        // .secondary-container {{
        // background-color: {secondary_container};
        // color: {secondary_container_text};
        // }}

        // .secondary-container-component {{
        // background-color: {secondary_container_component};
        // color: {secondary_container_component_text};
        // }}

        // .secondary-container-componenet-divider {{
        // background-color: {secondary_container_component_divider};
        // }}

        // * {{
        // background-image: none;
        // outline-color: {accent};
        // }}

        // button.suggested-action {{
        // background-color: {accent};
        // color: {suggested_button_text};
        // }}

        // button.destructive-action {{
        // background-color: {destructive};
        // outline-color: {destructive};
        // color: {destructive_button_text};
        // }}
        // "#
        // )
    }
}

impl TryFrom<(Selection, ThemeConstraints)> for Theme {
    type Error = anyhow::Error;

    fn try_from(
        (selection, constraints): (Selection, ThemeConstraints),
    ) -> Result<Self, Self::Error> {
        todo!()
    }
}
