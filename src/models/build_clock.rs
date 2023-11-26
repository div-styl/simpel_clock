use std::time::Duration;
use chrono::Local;
use glib::{timeout_add_local, ControlFlow};
use gtk4::{
    prelude::*, Application, ApplicationWindow, CssProvider, Label,
    STYLE_PROVIDER_PRIORITY_APPLICATION
};

pub fn clock(app: &Application) {
    let clock_label = Label::new(None);

    let win = ApplicationWindow::builder()
        .application(app)
        .title("Clock App")
        .child(&clock_label)
        .build();

    let css_provider = CssProvider::new();
    
    // Combine the CSS rules into a single string
    let css_data ="
        window { 
            background-color: #0F0F0F; 
            color: #363062;
            font-family: monospace;
            font-size: 40px;
            font-weight: bold;
        }
    ";
    
    css_provider.load_from_data(css_data);

    win.style_context()
        .add_provider(&css_provider, STYLE_PROVIDER_PRIORITY_APPLICATION);

    timeout_add_local(Duration::from_millis(100), move || {
        clock_label.set_text(&Local::now().time().format("%I:%M:%S %p").to_string());
        ControlFlow::Continue
    });

    // Ensure the window and its children are shown
    win.show();

    win.present();
}
