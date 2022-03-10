// SPDX-License-Identifier: GPL-3.0-only

use components::App;
use gtk4::{gdk::Display, prelude::*, Application, CssProvider, StyleContext};

mod color_picker;
mod components;
mod model;
mod util;

const APP_STRING: &'static str = "com.System76.CosmicThemeEditor";
const APP_TITLE: &'static str = "Cosmic Theme Editor";

fn setup_shortcuts(app: &Application) {
    //quit shortcut
    app.set_accels_for_action("win.quit", &["<primary>W", "Escape"]);
}

fn load_css() {
    // Load the css file and add it to the provider
    let provider = CssProvider::new();
    provider.load_from_data(include_bytes!("style.css"));

    // Add the provider to the default screen
    StyleContext::add_provider_for_display(
        &Display::default().expect("Error initializing GTK CSS provider."),
        &provider,
        gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}

fn main() {
    let app = gtk4::Application::builder()
        .application_id(APP_STRING)
        .build();

    app.connect_startup(|app| {
        setup_shortcuts(app);
        load_css();
    });
    app.connect_activate(move |app| {
        let theme_app = App::new(app);
        theme_app.show();
    });
    app.run();
}
