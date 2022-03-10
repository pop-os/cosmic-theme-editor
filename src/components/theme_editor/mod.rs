// SPDX-License-Identifier: GPL-3.0-only
use crate::model::Theme;
use cascade::cascade;
use gtk4::{
    builders::ButtonBuilder, gdk::Display, gio, glib, prelude::*, subclass::prelude::*, Box,
    Button, ColorButton, CssProvider, DialogFlags, Entry, Label, MessageDialog, MessageType,
    Orientation, ScrolledWindow, Separator, StyleContext, Window,
};
use relm4_macros::view;
use std::rc::Rc;
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
                        set_use_alpha: false,
                        add_css_class: "background_component",
                    },

                    append: background_color_label = &Label {
                        set_text: "Background Color",
                        add_css_class: "background_text",
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
                        add_css_class: "background_component",
                    },

                    append: primary_color_label = &Label {
                        set_text: "Primary Container Color",
                        add_css_class: "background_text",
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
                        add_css_class: "background_component",
                  },

                    append: secondary_color_label = &Label {
                        set_text: "Secondary Container Color",
                        add_css_class: "background_text",
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
                        add_css_class: "background_component",
                   },

                    append: accent_color_label = &Label {
                        set_text: "Accent Color",
                        add_css_class: "background_text",
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
                        add_css_class: "background_component",
                    },

                    append: accent_text_color_label = &Label {
                        set_text: "Accent Text Color",
                        add_css_class: "background_text",
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
                        add_css_class: "background_component",
                    },

                    append: accent_nav_handle_color_label = &Label {
                        set_text: "Accent Nav Handle Text Color",
                        add_css_class: "background_text",
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
                        add_css_class: "background_component",
                    },

                    append: destructive_color_label = &Label {
                        set_text: "Destructive Color",
                        add_css_class: "background_text",
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
                        add_css_class: "background_component",

                        set_child = Some(&Label) {
                            set_text: "Save",
                            set_margin_top: 4,
                            set_margin_bottom: 4,
                            set_margin_start: 4,
                            set_margin_end: 4,
                            add_css_class: "background_component_text",
                        }
                    },

                    append: preview_button = &Button {
                        set_margin_top: 4,
                        set_margin_bottom: 4,
                        set_margin_start: 4,
                        set_margin_end: 4,
                        add_css_class: "background_component",

                        set_child = Some(&Label) {
                            set_text: "Preview",
                            set_margin_top: 4,
                            set_margin_bottom: 4,
                            set_margin_start: 4,
                            set_margin_end: 4,
                            add_css_class: "background_component_text",
                        }
                    },
                },

                append: separator = &Separator {
                    set_orientation: Orientation::Horizontal,
                    set_margin_top: 4,
                    set_margin_bottom: 4,
                    set_margin_start: 4,
                    set_margin_end: 4,
                    add_css_class: "background_component_divider",
                },
                // TODO preview

                append = &Box {
                    set_hexpand: true,
                    set_height_request: 100,
                    add_css_class: "background",

                    append = &Label {
                        set_hexpand: true,
                        set_height_request: 50,
                        add_css_class: "background-component",
                        set_text: "Background Component"
                    }

                },

            }
        };

        let scroll_window = ScrolledWindow::builder()
            .hexpand(true)
            .vexpand(true)
            .child(&inner)
            .build();

        self_.append(&scroll_window);

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

        //TODO verify that gdk RGBA is Srgb

        imp.background_color_button
            .get()
            .unwrap()
            .connect_color_set(glib::clone!(@weak selection => move |self_| {
                let mut c = selection.get();
                c.set_background(self_.rgba());
                selection.set(c);
            }));

        imp.primary_color_button.get().unwrap().connect_color_set(
            glib::clone!(@weak selection => move |self_| {
                let mut c = selection.get();
                c.set_primary_container(self_.rgba());
                selection.set(c);
            }),
        );

        imp.secondary_color_button.get().unwrap().connect_color_set(
            glib::clone!(@weak selection => move |self_| {
                let mut c = selection.get();
                c.set_secondary_container(self_.rgba());
                selection.set(c);
            }),
        );

        imp.accent_text_color_button
            .get()
            .unwrap()
            .connect_color_set(glib::clone!(@weak selection => move |self_| {
                let mut c = selection.get();
                c.set_accent_text_color(self_.rgba());
                selection.set(c);
            }));

        imp.accent_color_button.get().unwrap().connect_color_set(
            glib::clone!(@weak selection => move |self_| {
                let mut c = selection.get();
                selection.get().set_accent_color(self_.rgba());
                selection.set(c);
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

        imp.save.get().unwrap().connect_clicked(
            glib::clone!(@weak selection, @weak theme, @weak constraints => move |_| {
                // TODO convert theme ron & css file
                // TODO convert constraints & selection to ron
                todo!();
            }),
        );

        imp.preview.get().unwrap().connect_clicked(
            glib::clone!(@weak selection, @weak theme, @weak constraints, @weak self as parent => move |self_| {
                // TODO convert constraints & selection to theme
                // TODO convert theme to CSS
                // TODO load css so that the preview displays the proper theme
                println!("generating new theme");
                if let Ok(new_theme) =  Theme::try_from((selection.get(), constraints.get())) {
                    dbg!(new_theme);
                    theme.set(new_theme);
                    let preview_css = theme.get().as_css();
                    dbg!(&preview_css);

                    let provider = CssProvider::new();
                    provider.load_from_data(preview_css.as_bytes());
                    StyleContext::add_provider_for_display(
                        &Display::default().expect("Error initializing GTK CSS provider."),
                        &provider,
                        gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION,
                    );

                } else {
                    eprintln!("failed to create new theme...");
                    let window = self_.root().map(|root| {
                        if let Ok(w) = root.downcast::<Window>() {
                            Some(w)
                        } else {
                            None
                        }
                    }).unwrap_or_default();
                    if let Some(window) = window {
                        glib::MainContext::default().spawn_local(Self::dialog(window));
                    }
                }
            }),
        );
    }

    async fn dialog(window: Window) {
        let msg_dialog = MessageDialog::builder()
            .transient_for(&window)
            .modal(true)
            .buttons(gtk4::ButtonsType::Close)
            .text("Could not generate theme given the selection and constraints")
            .build();
        let _ = msg_dialog.run_future().await;
        msg_dialog.close();
    }
}
