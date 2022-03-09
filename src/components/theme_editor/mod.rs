// SPDX-License-Identifier: GPL-3.0-only

use cascade::cascade;
use gtk4::{
    gio, glib, prelude::*, subclass::prelude::*, Box, Button, ColorButton, ColorChooser, Entry,
    Label, Orientation, Separator,
};
use relm4_macros::view;

mod imp;

glib::wrapper! {
    pub struct ThemeEditor(ObjectSubclass<imp::ThemeEditor>)
        @extends gtk4::Box, gtk4::Widget,
    @implements gtk4::Accessible, gtk4::Buildable, gtk4::ConstraintTarget, gtk4::Orientable;
}

impl Default for ThemeEditor {
    fn default() -> Self {
        Self::new()
    }
}

impl ThemeEditor {
    pub fn new() -> Self {
        let self_: Self = glib::Object::new(&[]).expect("Failed to create Theme Editor Widget");

        let imp = imp::ThemeEditor::from_instance(&self_);

        cascade! {
            &self_;
            ..set_orientation(Orientation::Vertical);
            ..set_hexpand(true);
            ..set_vexpand(true);
        };

        view! {
            inner = Box {
                set_orientation: Orientation::Vertical,
                set_spacing: 4,
                set_margin_top: 4,
                set_margin_bottom: 4,
                set_margin_start: 4,
                set_margin_end: 4,

                append: name = &Entry {
                    set_placeholder_text: Some("Theme Name"),
                    set_width_request: 160,
                },

                append: background_color_box = &Box {
                    set_orientation: Orientation::Horizontal,
                    set_spacing: 4,
                    set_margin_top: 4,
                    set_margin_bottom: 4,
                    set_margin_start: 4,
                    set_margin_end: 4,

                    append: background_color_button = &ColorButton {
                        set_title: "Background Color",
                    },

                    append: backgrounf_color_label = &Label {
                        set_text: "Background Color",
                    }
                },

                append: background_color_box = &Box {
                    set_orientation: Orientation::Horizontal,
                    set_spacing: 4,
                    set_margin_top: 4,
                    set_margin_bottom: 4,
                    set_margin_start: 4,
                    set_margin_end: 4,

                    append: background_color_button = &ColorButton {
                        set_title: "Background Color",
                        set_use_alpha: false,
                    },

                    append: background_color_label = &Label {
                        set_text: "Background Color",
                    }
                },

                append: primary_color_box = &Box {
                    set_orientation: Orientation::Horizontal,
                    set_spacing: 4,
                    set_margin_top: 4,
                    set_margin_bottom: 4,
                    set_margin_start: 4,
                    set_margin_end: 4,

                    append: primary_color_button = &ColorButton {
                        set_title: "Primary Container Color",
                        set_use_alpha: false,
                    },

                    append: primary_color_label = &Label {
                        set_text: "Primary Container Color",
                    }
                },

                append: secondary_color_box = &Box {
                    set_orientation: Orientation::Horizontal,
                    set_spacing: 4,
                    set_margin_top: 4,
                    set_margin_bottom: 4,
                    set_margin_start: 4,
                    set_margin_end: 4,

                    append: secondary_color_button = &ColorButton {
                        set_title: "Secondary Container Color",
                        set_use_alpha: false,
                  },

                    append: secondary_color_label = &Label {
                        set_text: "Secondary Container Color",
                  }
                },

                append: accent_color_box = &Box {
                    set_orientation: Orientation::Horizontal,
                    set_spacing: 4,
                    set_margin_top: 4,
                    set_margin_bottom: 4,
                    set_margin_start: 4,
                    set_margin_end: 4,

                    append: accent_color_button = &ColorButton {
                        set_title: "Accent Color",
                        set_use_alpha: false,
                   },

                    append: accent_color_label = &Label {
                        set_text: "Accent Color",
                   }
                },

                append: accent_text_color_box = &Box {
                    set_orientation: Orientation::Horizontal,
                    set_spacing: 4,
                    set_margin_top: 4,
                    set_margin_bottom: 4,
                    set_margin_start: 4,
                    set_margin_end: 4,

                    append: accent_text_color_button = &ColorButton {
                        set_title: "Accent Text Color",
                        set_use_alpha: false,
                    },

                    append: accent_text_color_label = &Label {
                        set_text: "Accent Text Color",
                    }
                },

                append: accent_nav_handle_color_box = &Box {
                    set_orientation: Orientation::Horizontal,
                    set_spacing: 4,
                    set_margin_top: 4,
                    set_margin_bottom: 4,
                    set_margin_start: 4,
                    set_margin_end: 4,

                    append: accent_nav_handle_color_button = &ColorButton {
                        set_title: "Accent Nav Handle Text Color",
                        set_use_alpha: false,
                    },

                    append: accent_nav_handle_color_label = &Label {
                        set_text: "Accent Nav Handle Text Color",
                    }
                },

                append: destructive_color_box = &Box {
                    set_orientation: Orientation::Horizontal,
                    set_spacing: 4,
                    set_margin_top: 4,
                    set_margin_bottom: 4,
                    set_margin_start: 4,
                    set_margin_end: 4,

                    append: destructive_color_button = &ColorButton {
                        set_title: "Destructive Color",
                    },

                    append: destructive_color_label = &Label {
                        set_text: "Destructive Color",
                    }
                },

                append: control_button_box = &Box {
                    set_orientation: Orientation::Horizontal,
                    set_spacing: 4,
                    set_margin_top: 4,
                    set_margin_bottom: 4,
                    set_margin_start: 4,
                    set_margin_end: 4,

                    append: save_button = &Button {
                        set_margin_top: 4,
                        set_margin_bottom: 4,
                        set_margin_start: 4,
                        set_margin_end: 4,

                        set_child = Some(&Label) {
                            set_text: "Save",
                            set_margin_top: 4,
                            set_margin_bottom: 4,
                            set_margin_start: 4,
                            set_margin_end: 4,
                        }
                    },

                    append: preview_button = &Button {
                        set_margin_top: 4,
                        set_margin_bottom: 4,
                        set_margin_start: 4,
                        set_margin_end: 4,

                        set_child = Some(&Label) {
                            set_text: "Preview",
                            set_margin_top: 4,
                            set_margin_bottom: 4,
                            set_margin_start: 4,
                            set_margin_end: 4,
                        }
                    },
                },

                append: separator = &Separator {
                    set_orientation: Orientation::Horizontal,
                    set_margin_top: 4,
                    set_margin_bottom: 4,
                    set_margin_start: 4,
                    set_margin_end: 4,
                }
                // TODO preview
            }
        };

        self_.append(&inner);

        // set widget state
        imp.name.set(name).unwrap();
        imp.save.set(save_button).unwrap();
        imp.preview.set(preview_button).unwrap();

        // color buttons
        imp.background_color_button
            .set(background_color_button)
            .unwrap();
        imp.primary_color_button.set(primary_color_button).unwrap();
        imp.secondary_color_button
            .set(secondary_color_button)
            .unwrap();
        imp.accent_text_color_button
            .set(accent_text_color_button)
            .unwrap();
        imp.accent_color_button.set(accent_color_button).unwrap();
        imp.accent_nav_handle_text_color_button
            .set(accent_nav_handle_color_button)
            .unwrap();
        imp.destructive_color_button
            .set(destructive_color_button)
            .unwrap();

        self_.connect_color_buttons();
        self_.connect_control_buttons();

        self_
    }

    fn connect_color_buttons(&self) {
        let imp = imp::ThemeEditor::from_instance(&self);
        let selection = &imp.selection;

        imp.background_color_button
            .get()
            .unwrap()
            .connect_color_set(glib::clone!(@weak selection => move |self_| {
                selection.get().set_background(self_.rgba());
            }));

        imp.primary_color_button.get().unwrap().connect_color_set(
            glib::clone!(@weak selection => move |self_| {
                selection.get().set_primary_container(self_.rgba());
            }),
        );

        imp.secondary_color_button.get().unwrap().connect_color_set(
            glib::clone!(@weak selection => move |self_| {
                selection.get().set_secondary_container(self_.rgba());
            }),
        );

        imp.accent_text_color_button
            .get()
            .unwrap()
            .connect_color_set(glib::clone!(@weak selection => move |self_| {
                selection.get().set_accent_text_color(self_.rgba());
            }));

        imp.accent_color_button.get().unwrap().connect_color_set(
            glib::clone!(@weak selection => move |self_| {
                selection.get().set_accent_color(self_.rgba());
            }),
        );

        imp.accent_nav_handle_text_color_button
            .get()
            .unwrap()
            .connect_color_set(glib::clone!(@weak selection => move |self_| {
                selection.get().set_accent_nav_handle_text_color(self_.rgba());
            }));

        imp.destructive_color_button
            .get()
            .unwrap()
            .connect_color_set(glib::clone!(@weak selection => move |self_| {
                selection.get().set_destructive(self_.rgba());
            }));
    }

    fn connect_control_buttons(&self) {
        let imp = imp::ThemeEditor::from_instance(&self);
        let selection = &imp.selection;
        let theme = &imp.theme;
        let constraints = &imp.constraints;

        imp.save.get().unwrap().connect_activate(
            glib::clone!(@weak selection, @weak theme, @weak constraints => move |_| {
                // TODO convert theme ron & css file
                // TODO convert constraints & selection to ron
                todo!();
            }),
        );

        imp.save.get().unwrap().connect_activate(
            glib::clone!(@weak selection, @weak theme, @weak constraints => move |_| {
                // TODO convert constraints & selection to theme
                // TODO convert theme to CSS
                // TODO load css so that the preview displays the proper theme
                todo!();
            }),
        );
    }
}
