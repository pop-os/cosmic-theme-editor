mod application;
mod components;
mod config;
mod util;
mod window;

use gettextrs::{gettext, LocaleCategory};
use gtk4::{gio, glib};

use self::application::ThemeEditorApplication;
use self::config::{GETTEXT_PACKAGE, LOCALEDIR, RESOURCES_FILE};

const APP_TITLE: &'static str = "Cosmic Theme Editor";

fn main() {
    // Initialize logger
    pretty_env_logger::init();

    // Prepare i18n
    gettextrs::setlocale(LocaleCategory::LcAll, "");
    gettextrs::bindtextdomain(GETTEXT_PACKAGE, LOCALEDIR).expect("Unable to bind the text domain");
    gettextrs::textdomain(GETTEXT_PACKAGE).expect("Unable to switch to the text domain");

    glib::set_application_name(&gettext("Cosmic Theme Editor"));

    let res = gio::Resource::load(RESOURCES_FILE).expect("Could not load gresource file");
    gio::resources_register(&res);

    let app = ThemeEditorApplication::new();
    app.run();
}
