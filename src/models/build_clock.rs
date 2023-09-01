use std::time::Duration;
/* use std::process::exit;*/
use chrono::Local;
use glib::{timeout_add_local, ControlFlow};
use gtk4::{
    prelude::*, Application, ApplicationWindow, CssProvider, Label,
    STYLE_PROVIDER_PRIORITY_APPLICATION
};
pub fn clock(app: &Application) {
    let clock_label = Label::new(None);
    // let button_exit = Button::with_label("Exit");
    //
    // button_exit.connect_clicked(|_| {
    //     exit(0);
    // });
    let win = ApplicationWindow::builder()
        .application(app)
        .title("Clock App")
        .child(&clock_label)
        /* .child(&button_exit) */
        .build();

    let css_provider = CssProvider::new();
    css_provider.load_from_data("window { background-color: #E2C799; }");
    win.style_context()
        .add_provider(&css_provider, STYLE_PROVIDER_PRIORITY_APPLICATION);

    timeout_add_local(Duration::from_millis(100), move || {
        clock_label.set_text(&Local::now().time().format("%I:%M:%S %p").to_string());
        ControlFlow::Continue
    });

    win.present();
}
