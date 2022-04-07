use gettextrs::gettext;
use log::{debug, info};

use glib::clone;
use gtk4::prelude::*;
use gtk4::subclass::prelude::*;
use gtk4::{gdk, gio, glib};

use crate::config::{APP_ID, PKGDATADIR, PROFILE, VERSION};
use crate::window::ThemeEditorApplicationWindow;

mod imp {
    use super::*;
    use glib::WeakRef;
    use once_cell::sync::OnceCell;

    #[derive(Debug, Default)]
    pub struct ThemeEditorApplication {
        pub window: OnceCell<WeakRef<ThemeEditorApplicationWindow>>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for ThemeEditorApplication {
        const NAME: &'static str = "ThemeEditorApplication";
        type Type = super::ThemeEditorApplication;
        type ParentType = gtk4::Application;
    }

    impl ObjectImpl for ThemeEditorApplication {}

    impl ApplicationImpl for ThemeEditorApplication {
        fn activate(&self, app: &Self::Type) {
            debug!("GtkApplication<ThemeEditorApplication>::activate");
            self.parent_activate(app);

            if let Some(window) = self.window.get() {
                let window = window.upgrade().unwrap();
                window.present();
                return;
            }

            let provider = app.setup_css();
            let window = ThemeEditorApplicationWindow::new(app, provider);
            self.window
                .set(window.downgrade())
                .expect("Window already set.");

            app.main_window().present();
        }

        fn startup(&self, app: &Self::Type) {
            debug!("GtkApplication<ThemeEditorApplication>::startup");
            self.parent_startup(app);

            // Set icons for shell
            gtk4::Window::set_default_icon_name(APP_ID);

            app.setup_gactions();
            app.setup_accels();
        }
    }

    impl GtkApplicationImpl for ThemeEditorApplication {}
}

glib::wrapper! {
    pub struct ThemeEditorApplication(ObjectSubclass<imp::ThemeEditorApplication>)
        @extends gio::Application, gtk4::Application,
        @implements gio::ActionMap, gio::ActionGroup;
}

impl ThemeEditorApplication {
    pub fn new() -> Self {
        glib::Object::new(&[
            ("application-id", &Some(APP_ID)),
            ("flags", &gio::ApplicationFlags::empty()),
            (
                "resource-base-path",
                &Some("/com/System76/CosmicThemeEditor/"),
            ),
        ])
        .expect("Application initialization failed...")
    }

    fn main_window(&self) -> ThemeEditorApplicationWindow {
        self.imp().window.get().unwrap().upgrade().unwrap()
    }

    fn setup_gactions(&self) {
        // Quit
        let action_quit = gio::SimpleAction::new("quit", None);
        action_quit.connect_activate(clone!(@weak self as app => move |_, _| {
            // This is needed to trigger the delete event and saving the window state
            app.main_window().close();
            app.quit();
        }));
        self.add_action(&action_quit);

        // About
        let action_about = gio::SimpleAction::new("about", None);
        action_about.connect_activate(clone!(@weak self as app => move |_, _| {
            app.show_about_dialog();
        }));
        self.add_action(&action_about);
    }

    // Sets up keyboard shortcuts
    fn setup_accels(&self) {
        self.set_accels_for_action("app.quit", &["<Control>q"]);
    }

    fn setup_css(&self) -> gtk4::CssProvider {
        let provider = gtk4::CssProvider::new();
        gtk4::StyleContext::add_provider_for_display(
            &gdk::Display::default().expect("Error initializing GTK CSS provider."),
            &provider,
            gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );
        let provider_clone = provider.clone();
        glib::MainContext::default().spawn_local(async move {
            if let Err(e) = cosmic_theme::load_cosmic_gtk4_theme(provider_clone).await {
                eprintln!("{}", e);
            }
        });
        provider
    }

    fn show_about_dialog(&self) {
        let dialog = gtk4::AboutDialog::builder()
            .logo_icon_name(APP_ID)
            // Insert your license of choice here
            // .license_type(gtk::License::MitX11)
            // Insert your website here
            // .website("https://gitlab.gnome.org/bilelmoussaoui/cosmic-theme-editor/")
            .version(VERSION)
            .transient_for(&self.main_window())
            .translator_credits(&gettext("translator-credits"))
            .modal(true)
            .authors(vec!["Ashley Wulber".into()])
            .artists(vec!["Ashley Wulber".into()])
            .build();

        dialog.present();
    }

    pub fn run(&self) {
        info!("Cosmic Theme Editor ({})", APP_ID);
        info!("Version: {} ({})", VERSION, PROFILE);
        info!("Datadir: {}", PKGDATADIR);

        ApplicationExtManual::run(self);
    }
}
