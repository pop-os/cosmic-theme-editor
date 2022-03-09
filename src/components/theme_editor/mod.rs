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

                    append: background_color_picker_button = &ColorButton {
                        set_title: "Background Color",
                    },

                    append: backgrounf_color_picker_label = &Label {
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

                    append: background_color_picker_button = &ColorButton {
                        set_title: "Background Color",
                    },

                    append: background_color_picker_label = &Label {
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

                    append: primary_color_picker_button = &ColorButton {
                        set_title: "Background Color",
                    },

                    append: primary_color_picker_label = &Label {
                        set_text: "Background Color",
                    }
                },

                append: secondary_color_box = &Box {
                    set_orientation: Orientation::Horizontal,
                    set_spacing: 4,
                    set_margin_top: 4,
                    set_margin_bottom: 4,
                    set_margin_start: 4,
                    set_margin_end: 4,

                    append: secondary_color_picker_button = &ColorButton {
                        set_title: "Background Color",
                        set_margin_top: 4,
                        set_margin_bottom: 4,
                        set_margin_start: 4,
                        set_margin_end: 4,
                   },

                    append: secondary_color_picker_label = &Label {
                        set_text: "Background Color",
                        set_margin_top: 4,
                        set_margin_bottom: 4,
                        set_margin_start: 4,
                        set_margin_end: 4,
                   }
                },

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

        // TODO set up handlers
        // set widget state

        self_
    }
}
