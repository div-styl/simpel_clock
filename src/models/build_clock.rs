use gtk4::prelude::*;
use gtk4::{Application, ApplicationWindow};
//use std::process::exit;

pub fn clock(app: &Application ){
    //create the Button
    // let button1 = Button::builder()
    //     .label("Welcome to GTK")
    //     .margin_top(10)
    //     .margin_bottom(10)
    //     .margin_start(5)
    //     .margin_end(5)
    //     .width_request(100)
    //     .build();
    // //let button2
    // // connect the Button
    // button1.connect_clicked(|button1: &Button|{
    //     button1.set_label("Noob");
    //     eprintln!("Button is killed");
    //     exit(0);
    // });

    // create ApplicationWindow
    let win = ApplicationWindow::builder()
        .application(app)
        .title("into")
        .build();
    // apply Css
    let css_provider = gtk4::CssProvider::new();
    css_provider
        .load_from_data("window { background-color: #183D3D; }");
    let style_context = win.style_context();
    style_context.add_provider(&css_provider, gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION);

    // Present window
    win.present();
}
