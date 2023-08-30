use gtk4 as gtk;
use gtk::{Application, glib, prelude::*};
use crate::models::build_clock;

const APP_ID: &str = "simple_clock";

pub fn launch_win() -> glib::ExitCode {
    // Run the app
    let app = Application::builder().application_id(APP_ID).build();

    // Corrected method name: connect_activate
    app.connect_activate(|app| {
        build_clock::clock(app);
    });

    // Run the application
    app.run()
}

