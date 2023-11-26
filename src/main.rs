use gtk4 as gtk;
use gtk::glib;

mod models;
use models::launch_win;

//use models::time;
fn main () -> glib::ExitCode {
   
    launch_win::launch_win();
    //time::timer();
    glib::ExitCode::SUCCESS
}
