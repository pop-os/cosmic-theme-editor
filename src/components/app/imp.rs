// SPDX-License-Identifier: GPL-3.0-only

use gtk4::{gio, glib, subclass::prelude::*, Entry};
use std::cell::Cell;

use crate::components::ThemeEditor;

// Object holding the state
#[derive(Default)]
pub struct App {
    pub theme_editor: Cell<ThemeEditor>,
}

// The central trait for subclassing a GObject
#[glib::object_subclass]
impl ObjectSubclass for App {
    const NAME: &'static str = "ThemeEditorApp";
    type Type = super::App;
    type ParentType = gtk4::ApplicationWindow;
}

// Trait shared by all GObjects
impl ObjectImpl for App {}

// Trait shared by all widgets
impl WidgetImpl for App {}

// Trait shared by all windows
impl WindowImpl for App {}

// Trait shared by all application
impl ApplicationWindowImpl for App {}
