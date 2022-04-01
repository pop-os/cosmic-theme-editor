use cascade::cascade;
use gtk4::{gio, glib, prelude::*, subclass::prelude::*, Application, CssProvider};

use crate::{
    application::ThemeEditorApplication,
    components::ThemeEditor,
    config::{APP_ID, PROFILE},
};

mod imp {
    use super::*;

    use std::cell::Cell;

    use crate::components::ThemeEditor;

    // Object holding the state
    #[derive(Default)]
    pub struct ThemeEditorApplicationWindow {
        pub theme_editor: Cell<Option<ThemeEditor>>,
    }

    // The central trait for subclassing a GObject
    #[glib::object_subclass]
    impl ObjectSubclass for ThemeEditorApplicationWindow {
        const NAME: &'static str = "ThemeEditorApplicationWindow";
        type Type = super::ThemeEditorApplicationWindow;
        type ParentType = gtk4::ApplicationWindow;
    }

    // Trait shared by all GObjects
    impl ObjectImpl for ThemeEditorApplicationWindow {}

    // Trait shared by all widgets
    impl WidgetImpl for ThemeEditorApplicationWindow {}

    // Trait shared by all windows
    impl WindowImpl for ThemeEditorApplicationWindow {}

    // Trait shared by all application
    impl ApplicationWindowImpl for ThemeEditorApplicationWindow {}
}

glib::wrapper! {
    pub struct ThemeEditorApplicationWindow(ObjectSubclass<imp::ThemeEditorApplicationWindow>)
        @extends gtk4::ApplicationWindow, gtk4::Window, gtk4::Widget,
    @implements gio::ActionGroup, gio::ActionMap, gtk4::Accessible, gtk4::Buildable,
    gtk4::ConstraintTarget, gtk4::Native, gtk4::Root, gtk4::ShortcutManager;
}

impl ThemeEditorApplicationWindow {
    pub fn new(app: &ThemeEditorApplication, provider: CssProvider) -> Self {
        let self_: Self = glib::Object::new(&[("application", app)])
            .expect("Failed to create Theme Editor Application");
        let imp = imp::ThemeEditorApplicationWindow::from_instance(&self_);

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
