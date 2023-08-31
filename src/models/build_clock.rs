use gtk4::prelude::*;
use gtk4::{Application, ApplicationWindow};
use crate::models::time::timer;
use std::sync::mpsc;
use std::thread;


pub fn clock(app: &Application) {
    let clock_label = gtk4::Label::new(None);

    // Create ApplicationWindow
    let win = ApplicationWindow::builder()
        .application(app)
        .title("Clock App")
        .child(&clock_label) // Add the label to the window
        .build();

    // Create a channel to send updates from the timer thread
    let (tx, rx) = mpsc::channel();

    // Clone the label for use in the closure
    let label_clone = clock_label.clone();

    // Clone the sender for the timer thread
    let sender_clone = tx.clone();

    // Apply CSS
    let css_provider = gtk4::CssProvider::new();
    css_provider.load_from_data("window { background-color: #183D3D; }");
    let style_context = win.style_context();
    style_context.add_provider(&css_provider, gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION);

    // Spawn a new thread to update the clock label
    thread::spawn(move || {
        timer(sender_clone);
    });

    // Listen for updates from the timer thread and update the label
    glib::timeout_add_seconds_local(1, move || {
        if let Ok(formatted_time) = rx.try_recv() {
            label_clone.set_text(&formatted_time);
        }
        std::ops::ControlFlow::Continue(true)
    });

    // Present window
    win.present();
}