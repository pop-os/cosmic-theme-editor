// SPDX-License-Identifier: GPL-3.0-only

use gtk4::{
    gio,
    glib::{self, subclass::Signal},
    prelude::*,
    subclass::prelude::*,
    Box, Button, FileChooserNative,
};
use once_cell::sync::Lazy;
use std::{cell::RefCell, rc::Rc};

// Object holding the state
#[derive(Default)]
pub struct FileButton {
    pub button: Rc<RefCell<Button>>,
    pub file_chooser: Rc<RefCell<FileChooserNative>>,
}

#[glib::object_subclass]
impl ObjectSubclass for FileButton {
    const NAME: &'static str = "FileButton";
    type Type = super::FileButton;
    type ParentType = Box;
}

// Trait shared by all GObjects
impl ObjectImpl for FileButton {
    fn signals() -> &'static [Signal] {
        static SIGNALS: Lazy<Vec<Signal>> = Lazy::new(|| {
            vec![Signal::builder(
                // Signal name
                "image-selected",
                // Types of the values which will be sent to the signal handler
                &[gio::File::static_type().into()],
                // Type of the value the signal handler sends back
                <()>::static_type().into(),
            )
            .build()]
        });
        SIGNALS.as_ref()
    }
}

// Trait shared by all widgets
impl WidgetImpl for FileButton {}

// Trait shared by all boxes
impl BoxImpl for FileButton {}
