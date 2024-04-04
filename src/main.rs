use gtk::gio;
use gtk::glib::{self};
use gtk::prelude::*;
use gtk::Application;

mod task_object;
mod task_row;
mod window;

use window::Window;

const APP_ID: &str = "org.gtk_rs.todo";

fn main() -> glib::ExitCode {
    gio::resources_register_include!("resources.gresource").expect("Failed to register resources.");

    let app = Application::builder().application_id(APP_ID).build();
    app.connect_activate(build_ui);
    app.run()
}

fn build_ui(app: &Application) {
    let window = Window::new(app);
    window.present();
}
