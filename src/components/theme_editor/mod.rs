// SPDX-License-Identifier: GPL-3.0-only

use crate::{components::FileButton, util::SRGBA};

use cascade::cascade;
use cosmic_theme::{ColorPicker, Exact, Gtk4Output, Derivation};
use gettextrs::gettext;
use gtk4::{
    gio::File,
    glib::{self, closure_local},
    prelude::*,
    subclass::prelude::*,
    Box, Button, ColorButton, CssProvider, Entry, Label, MessageDialog, Orientation,
    ScrolledWindow, Separator, Switch, Window,
};
use relm4_macros::view;
use std::fmt::Display;
mod imp;

glib::wrapper! {
    pub struct ThemeEditor(ObjectSubclass<imp::ThemeEditor>)
        @extends gtk4::Box, gtk4::Widget,
    @implements gtk4::Accessible, gtk4::Buildable, gtk4::ConstraintTarget, gtk4::Orientable;
}

impl ThemeEditor {
    pub fn new(provider: CssProvider) -> Self {
        let self_: Self = glib::Object::new(&[]).expect("Failed to create Theme Editor Widget");

        let imp = imp::ThemeEditor::from_instance(&self_);

        cascade! {
            &self_;
            ..set_orientation(Orientation::Vertical);
        };

        let (background_color_box, background_color_button) =
            Self::get_color_button(&gettext("Background Color"));
        let (primary_color_box, primary_color_button) =
            Self::get_color_button(&gettext("Primary Container Color"));
        let (secondary_color_box, secondary_color_button) =
            Self::get_color_button(&gettext("Secondary Container Color"));
        let (accent_color_box, accent_color_button) =
            Self::get_color_button(&gettext("Accent Color"));
        let (accent_text_color_box, accent_text_color_button) =
            Self::get_color_button(&gettext("Accent Text Color"));
        let (accent_nav_handle_color_box, accent_nav_handle_color_button) =
            Self::get_color_button(&gettext("Accent Nav Text Color"));
        let (destructive_color_box, destructive_color_button) =
            Self::get_color_button(&gettext("Destructive Color"));

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
                    set_margin_top: 4,
                    set_margin_bottom: 4,
                    set_margin_start: 4,
                    set_margin_end: 4,
                    add_css_class: "background-component",
                    add_css_class: "padding-medium",
                    add_css_class: "border-radius-medium",
                    set_width_request: 160,
                },

                append = &Box {
                    set_orientation: Orientation::Horizontal,
                    set_spacing: 4,
                    set_margin_top: 4,
                    set_margin_bottom: 4,
                    set_margin_start: 4,
                    set_margin_end: 4,

                    append: lighten_elevated_surfaces = &Switch {
                        set_active: imp.constraints.get().lighten,
                        set_margin_top: 4,
                        set_margin_bottom: 4,
                        set_margin_start: 4,
                        set_margin_end: 4,
                        add_css_class: "background-component",

                    },

                    append = &Label {
                        set_text: &gettext("Lighten Elevated Surfaces"),
                    }
                },

                append: &background_color_box,
                append: &primary_color_box,
                append: &secondary_color_box,
                append: &accent_color_box,
                append: &accent_text_color_box,
                append: &accent_nav_handle_color_box,
                append: &destructive_color_box,

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
                        add_css_class: "background-component",
                        add_css_class: "padding-medium",
                        add_css_class: "border-radius-medium",

                        set_child = Some(&Label) {
                            set_text: &gettext("Save"),
                        }
                    },

                    append: preview_button = &Button {
                        set_margin_top: 4,
                        set_margin_bottom: 4,
                        set_margin_start: 4,
                        set_margin_end: 4,
                        add_css_class: "background-component",
                        add_css_class: "padding-medium",
                        add_css_class: "border-radius-medium",

                        set_child = Some(&Label) {
                            set_text: &gettext("Preview"),
                        }
                    },

                    append: file_button = &FileButton {},
                },


                // PREVIEW
                append: separator = &Separator {
                    set_orientation: Orientation::Horizontal,
                    set_margin_top: 8,
                    set_margin_bottom: 8,
                    set_margin_start: 8,
                    set_margin_end: 8,
                    add_css_class: "background-divider",
               },

                append = &Box {
                    set_orientation: Orientation::Horizontal,
                    set_spacing: 4,
                    set_margin_top: 4,
                    set_margin_bottom: 4,
                    set_margin_start: 4,
                    set_margin_end: 4,

                    append = &Button {
                        set_margin_top: 4,
                        set_margin_bottom: 4,
                        set_margin_start: 4,
                        set_margin_end: 4,
                        add_css_class: "destructive-action",
                        add_css_class: "padding-medium",
                        add_css_class: "border-radius-medium",

                        set_child = Some(&Label) {
                            set_text: &gettext("Destructive"),
                            set_margin_top: 4,
                            set_margin_bottom: 4,
                            set_margin_start: 4,
                            set_margin_end: 4,
                        }
                    },

                    append = &Button {
                        set_margin_top: 4,
                        set_margin_bottom: 4,
                        set_margin_start: 4,
                        set_margin_end: 4,
                        add_css_class: "suggested-action",
                        add_css_class: "padding-medium",
                        add_css_class: "border-radius-medium",

                        set_child = Some(&Label) {
                            set_text: &gettext("Suggested"),
                            set_margin_top: 4,
                            set_margin_bottom: 4,
                            set_margin_start: 4,
                            set_margin_end: 4,
                        }
                    },
                },

                append = &Box {
                    set_orientation: Orientation::Vertical,
                    set_hexpand: true,
                    set_height_request: 100,
                    add_css_class: "background",
                    add_css_class: "padding-medium",
                    add_css_class: "border-radius-medium",
                    set_margin_top: 8,
                    set_margin_bottom: 8,
                    set_margin_start: 8,
                    set_margin_end: 8,

                    append = &Label {
                        set_hexpand: true,
                        set_margin_top: 8,
                        set_margin_bottom: 8,
                        set_margin_start: 8,
                        set_margin_end: 8,
                        set_text: &gettext("Background")
                    },

                    append = &Box {
                        set_orientation: Orientation::Vertical,
                        set_hexpand: true,
                        set_height_request: 50,
                        add_css_class: "padding-medium",
                        add_css_class: "border-radius-medium",
                        add_css_class: "background-component",
                        set_margin_top: 8,
                        set_margin_bottom: 8,
                        set_margin_start: 8,
                        set_margin_end: 8,

                        append = &Label {
                            set_hexpand: true,
                            set_margin_top: 8,
                            set_margin_bottom: 8,
                            set_margin_start: 8,
                            set_margin_end: 8,
                            set_text: &gettext("Background Component")
                        },
                    },

                    append = &Separator {
                        set_orientation: Orientation::Horizontal,
                        add_css_class: "background-divider",
                        set_margin_top: 8,
                        set_margin_bottom: 8,
                        set_margin_start: 8,
                        set_margin_end: 8,
                    },

                    append = &Box {
                        set_orientation: Orientation::Vertical,
                        set_hexpand: true,
                        set_height_request: 100,
                        add_css_class: "primary-container",
                        add_css_class: "padding-medium",
                        add_css_class: "border-radius-medium",
                       set_margin_top: 8,
                        set_margin_bottom: 8,
                        set_margin_start: 8,
                        set_margin_end: 8,

                        append = &Label {
                            set_hexpand: true,
                            set_margin_top: 8,
                            set_margin_bottom: 8,
                            set_margin_start: 8,
                            set_margin_end: 8,
                            set_text: &gettext("Primary Container")
                        },

                        append = &Box {
                            set_orientation: Orientation::Vertical,
                            set_hexpand: true,
                            set_height_request: 50,
                            add_css_class: "primary-container-component",
                            add_css_class: "padding-medium",
                            add_css_class: "border-radius-medium",
                           set_margin_top: 8,
                            set_margin_bottom: 8,
                            set_margin_start: 8,
                            set_margin_end: 8,

                            append = &Label {
                                set_hexpand: true,
                                set_margin_top: 8,
                                set_margin_bottom: 8,
                                set_margin_start: 8,
                                set_margin_end: 8,
                                set_text: &gettext("Primary Container Component")
                            },
                        },

                        append = &Separator {
                            set_orientation: Orientation::Horizontal,
                            add_css_class: "primary-container-divider",
                            set_margin_top: 8,
                            set_margin_bottom: 8,
                            set_margin_start: 8,
                            set_margin_end: 8,
                        },

                        append = &Box {
                            set_orientation: Orientation::Vertical,
                            set_hexpand: true,
                            set_height_request: 100,
                            add_css_class: "secondary-container",
                            add_css_class: "padding-medium",
                            add_css_class: "border-radius-medium",
                           set_margin_top: 8,
                            set_margin_bottom: 8,
                            set_margin_start: 8,
                            set_margin_end: 8,

                            append = &Label {
                                set_hexpand: true,
                                set_margin_top: 8,
                                set_margin_bottom: 8,
                                set_margin_start: 8,
                                set_margin_end: 8,
                                set_text: &gettext("Secondary Container")
                            },

                            append = &Box {
                                set_orientation: Orientation::Vertical,
                                set_hexpand: true,
                                set_height_request: 50,
                                add_css_class: "secondary-container-component",
                                add_css_class: "padding-medium",
                                add_css_class: "border-radius-medium",
                               set_margin_top: 8,
                                set_margin_bottom: 8,
                                set_margin_start: 8,
                                set_margin_end: 8,

                                append = &Label {
                                    set_hexpand: true,
                                    set_margin_top: 8,
                                    set_margin_bottom: 8,
                                    set_margin_start: 8,
                                    set_margin_end: 8,
                                    set_text: &gettext("Secondary Container Component")
                                },
                            },

                            append = &Separator {
                                set_orientation: Orientation::Horizontal,
                                add_css_class: "secondary-container-divider",
                                set_margin_top: 8,
                                set_margin_bottom: 8,
                                set_margin_start: 8,
                                set_margin_end: 8,
                            },
                        },
                    },
                },
            }
        };

        let scroll_window = ScrolledWindow::builder()
            .hexpand(true)
            .vexpand(true)
            .child(&inner)
            .build();

        self_.append(&scroll_window);

        imp.css_provider.set(provider).unwrap();

        // set widget state
        imp.name.set(name).unwrap();
        imp.save.set(save_button).unwrap();
        imp.preview.set(preview_button).unwrap();
        imp.file_button.set(file_button).unwrap();

        imp.lighten_elevated_surfaces
            .set(lighten_elevated_surfaces)
            .unwrap();
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

        self_.connect_name();
        self_.connect_color_buttons();
        self_.connect_control_buttons();
        self_.connect_toggle();
        self_.connect_file_button();

        self_
    }

    fn connect_name(&self) {
        let imp = imp::ThemeEditor::from_instance(&self);
        imp.name.get().unwrap().connect_changed(
            glib::clone!(@weak imp.theme as theme => move |name| {
                let name = name.text();
                theme.borrow_mut().name = String::from(name.as_str());
            }),
        );
    }

    fn connect_file_button(&self) {
        let imp = imp::ThemeEditor::from_instance(&self);
        imp.file_button.get().unwrap().connect_closure(
            "image-selected",
            false,
            closure_local!(@weak-allow-none imp.selection as selection, @weak-allow-none self as self_ => move |_file_button: FileButton, f: File| {
                dbg!(f.path());
                if let Some(Ok(s)) = cosmic_theme::palette_from_image(f.path().unwrap()).map(|f| f.try_into()) {
                    selection.unwrap().set(s);
                    self_.unwrap().update_color_buttons();
                }
            }),
        );
    }

    fn update_color_buttons(&self) {
        let imp = imp::ThemeEditor::from_instance(&self);
        let selection = imp.selection.get();
        imp.background_color_button
            .get()
            .unwrap()
            .set_rgba(&selection.background.into());
        imp.primary_color_button
            .get()
            .unwrap()
            .set_rgba(&selection.primary_container.into());
        imp.secondary_color_button
            .get()
            .unwrap()
            .set_rgba(&selection.secondary_container.into());
        imp.accent_color_button
            .get()
            .unwrap()
            .set_rgba(&selection.accent.into());
        imp.accent_text_color_button
            .get()
            .unwrap()
            .set_rgba(&selection.accent_text.unwrap_or_default().into());
        imp.accent_nav_handle_text_color_button
            .get()
            .unwrap()
            .set_rgba(&selection.accent_nav_handle_text.unwrap_or_default().into());
        imp.destructive_color_button
            .get()
            .unwrap()
            .set_rgba(&selection.destructive.into());
    }

    fn connect_toggle(&self) {
        let imp = imp::ThemeEditor::from_instance(&self);
        let constraints = &imp.constraints;

        imp.lighten_elevated_surfaces
            .get()
            .unwrap()
            .connect_activate(glib::clone!(@weak constraints => move |toggle| {
                let mut c = constraints.get();
                c.lighten = toggle.is_active();
                constraints.set(c);
            }));
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
                c.background = SRGBA::from(self_.rgba());
                selection.set(c);
            }));

        imp.primary_color_button.get().unwrap().connect_color_set(
            glib::clone!(@weak selection => move |self_| {
                let mut c = selection.get();
                c.primary_container = SRGBA::from(self_.rgba());
                selection.set(c);
            }),
        );

        imp.secondary_color_button.get().unwrap().connect_color_set(
            glib::clone!(@weak selection => move |self_| {
                let mut c = selection.get();
                c.secondary_container = SRGBA::from(self_.rgba());
                selection.set(c);
            }),
        );

        imp.accent_text_color_button
            .get()
            .unwrap()
            .connect_color_set(glib::clone!(@weak selection => move |self_| {
                let mut c = selection.get();
                c.accent_text = Some(self_.rgba().into());
                selection.set(c);
            }));

        imp.accent_color_button.get().unwrap().connect_color_set(
            glib::clone!(@weak selection => move |self_| {
                let mut c = selection.get();
                c.accent = self_.rgba().into();
                selection.set(c);
            }),
        );

        imp.accent_nav_handle_text_color_button
            .get()
            .unwrap()
            .connect_color_set(glib::clone!(@weak selection => move |self_| {
                let mut c = selection.get();
                c.accent_nav_handle_text = Some(self_.rgba().into());
                selection.set(c);
            }));

        imp.destructive_color_button
            .get()
            .unwrap()
            .connect_color_set(glib::clone!(@weak selection => move |self_| {
                let mut c = selection.get();
                c.destructive = self_.rgba().into();
                selection.set(c)
            }));
    }

    fn get_color_button(label: &str) -> (Box, ColorButton) {
        let rgba = SRGBA::default().into();
        let color_button = cascade! {
            ColorButton::with_rgba(&rgba);
            ..set_title(label);
            ..set_use_alpha(true);
            ..add_css_class("background-component");
        };
        view! {
            color_box = Box {
                set_orientation: Orientation::Horizontal,
                set_spacing: 4,
                set_margin_top: 4,
                set_margin_bottom: 4,
                set_margin_start: 4,
                set_margin_end: 4,

                append: &color_button,

                append: accent_color_label = &Label {
                    set_text: label,
                    add_css_class: "background-text",
                }
            }
        };
        (color_box, color_button)
    }

    fn connect_control_buttons(&self) {
        let imp = imp::ThemeEditor::from_instance(&self);
        let selection = &imp.selection;
        let theme = &imp.theme;
        let constraints = &imp.constraints;
        let css_provider = &imp.css_provider;

        imp.save.get().unwrap().connect_clicked(
            glib::clone!(@weak selection, @weak theme, @weak constraints => move |save| {
                println!("saving the theme...");
                if theme.borrow().name != "" {
                    if let Err(e) = theme.borrow().write() {
                        let window = save.root().map(|root| {
                            if let Ok(w) = root.downcast::<Window>() {
                                Some(w)
                            } else {
                                None
                            }
                        }).unwrap_or_default();
                        if let Some(window) = window {
                            glib::MainContext::default().spawn_local(Self::dialog(window, format!("{}", e)));
                        }

                    }
                }
            }),
        );

        imp.preview.get().unwrap().connect_clicked(
            glib::clone!(@weak selection, @weak theme, @weak constraints, @weak css_provider, @weak self as parent => move |self_| {
                println!("generating new theme");
                let picker = Exact::new(selection.get(), constraints.get());
                let Derivation {derived: new_theme, errors} = picker.theme_derivation();
                    dbg!(&new_theme);
                    theme.replace(new_theme);
                    let preview_css = theme.borrow().as_css();
                    println!("{}", &preview_css);

                css_provider.get().unwrap().load_from_data(preview_css.as_bytes());
                if errors.len() > 0 {
                    eprintln!("Errors while creating new theme...");
                    let window = self_.root().map(|root| {
                        if let Ok(w) = root.downcast::<Window>() {
                            Some(w)
                        } else {
                            None
                        }
                    }).unwrap_or_default();
                    let err = errors.iter().map(|e| format!("{}", e)).collect::<Vec<String>>().join("\n\n");
                    if let Some(window) = window {
                        glib::MainContext::default().spawn_local(Self::dialog(window, err));
                    }
                }
            }),
        );
    }

    async fn dialog<T: Display>(window: Window, msg: T) {
        let msg_dialog = MessageDialog::builder()
            .transient_for(&window)
            .modal(true)
            .buttons(gtk4::ButtonsType::Close)
            .text(&format!("{}", msg))
            .build();
        cascade! {
            msg_dialog.message_area();
            ..set_margin_top(8);
            ..set_margin_bottom(8);
            ..set_margin_start(8);
            ..set_margin_end(8);
        };
        let _ = msg_dialog.run_future().await;
        msg_dialog.close();
    }
}
