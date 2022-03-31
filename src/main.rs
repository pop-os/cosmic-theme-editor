// SPDX-License-Identifier: GPL-3.0-only

use components::App;
use cosmic_theme::Config;
use gtk4::{gdk::Display, glib, prelude::*, Application, CssProvider, StyleContext};

mod components;
mod util;

const APP_STRING: &'static str = "com.System76.CosmicThemeEditor";
const APP_TITLE: &'static str = "Cosmic Theme Editor";

fn setup_shortcuts(app: &Application) {
    //quit shortcut
    app.set_accels_for_action("win.quit", &["<primary>W", "Escape"]);
}

fn load_css() -> CssProvider {
    let provider = CssProvider::new();
    StyleContext::add_provider_for_display(
        &Display::default().expect("Error initializing GTK CSS provider."),
        &provider,
        gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
    let provider_clone = provider.clone();
    glib::MainContext::default().spawn_local(async move {
        if let Err(e) = cosmic_theme::load_cosmic_gtk_theme(provider_clone).await {
            eprintln!("{}", e);
        }
    });
    provider
}

fn main() {
    let app = gtk4::Application::builder()
        .application_id(APP_STRING)
        .build();

    let config = Config::new("bubbleine-light".into(), "bubbleine-dark".into());
    config.save().unwrap();
    app.connect_startup(|app| {
        setup_shortcuts(app);
    });
    app.connect_activate(move |app| {
        let provider = load_css();
        let theme_app = App::new(app, provider);
        theme_app.show();
    });
    app.run();
}
