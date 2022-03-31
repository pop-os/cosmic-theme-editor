// SPDX-License-Identifier: GPL-3.0-only

use super::ThemeEditor;
use cascade::cascade;
use gtk4::{gio, glib, prelude::*, subclass::prelude::*, Application, CssProvider};

mod imp;

glib::wrapper! {
    pub struct App(ObjectSubclass<imp::App>)
        @extends gtk4::ApplicationWindow, gtk4::Window, gtk4::Widget,
    @implements gio::ActionGroup, gio::ActionMap, gtk4::Accessible, gtk4::Buildable,
    gtk4::ConstraintTarget, gtk4::Native, gtk4::Root, gtk4::ShortcutManager;
}

impl App {
    pub fn new(app: &Application, provider: CssProvider) -> Self {
        let self_: Self = glib::Object::new(&[("application", app)])
            .expect("Failed to create Theme Editor Application");
        let imp = imp::App::from_instance(&self_);

        cascade! {
            &self_;
            ..set_width_request(500);
            ..set_height_request(500);
            ..set_title(Some(crate::APP_TITLE));
            ..add_css_class("background");
        };

        let theme_editor = ThemeEditor::new(provider);
        self_.set_child(Some(&theme_editor));
        imp.theme_editor.set(Some(theme_editor));
        self_
    }
}
