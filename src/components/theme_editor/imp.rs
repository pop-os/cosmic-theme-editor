// SPDX-License-Identifier: GPL-3.0-only

use gtk4::{glib, subclass::prelude::*, Button, ColorButton, Entry, Switch};
use once_cell::sync::OnceCell;
use std::{cell::Cell, rc::Rc};

use crate::{
    components::FileButton,
    model::{Selection, Theme, ThemeConstraints},
};

// Object holding the state
#[derive(Default)]
pub struct ThemeEditor {
    pub name: OnceCell<Entry>,
    pub save: OnceCell<Button>,
    pub preview: OnceCell<Button>,
    pub file_button: OnceCell<FileButton>,
    pub lighten_elevated_surfaces: OnceCell<Switch>,
    pub background_color_button: OnceCell<ColorButton>,
    pub primary_color_button: OnceCell<ColorButton>,
    pub secondary_color_button: OnceCell<ColorButton>,
    pub accent_color_button: OnceCell<ColorButton>,
    pub accent_text_color_button: OnceCell<ColorButton>,
    pub accent_nav_handle_text_color_button: OnceCell<ColorButton>,
    pub destructive_color_button: OnceCell<ColorButton>,
    pub constraints: Rc<Cell<ThemeConstraints>>,
    pub selection: Rc<Cell<Selection>>,
    pub theme: Rc<Cell<Theme>>,
}

// The central trait for subclassing a GObject
#[glib::object_subclass]
impl ObjectSubclass for ThemeEditor {
    const NAME: &'static str = "ThemeEditorWidget";
    type Type = super::ThemeEditor;
    type ParentType = gtk4::Box;
}

// Trait shared by all GObjects
impl ObjectImpl for ThemeEditor {}

// Trait shared by all widgets
impl WidgetImpl for ThemeEditor {}

// Trait shared by all boxes
impl BoxImpl for ThemeEditor {}
