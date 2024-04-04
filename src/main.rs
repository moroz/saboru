use gtk::glib::{self};
use gtk::{gio, ApplicationWindow};
use gtk::{prelude::*, Label, ListBox, PolicyType, ScrolledWindow};
use gtk::{Application, Orientation};

const APP_ID: &str = "org.gtk_rs.todo";

fn main() -> glib::ExitCode {
    let app = Application::builder().application_id(APP_ID).build();
    app.connect_activate(build_ui);
    app.run()
}

fn build_ui(app: &Application) {
    let list_box = ListBox::new();
    for number in 0..=100 {
        let label = Label::new(Some(&number.to_string()));
        list_box.append(&label);
    }

    let scrolled_window = ScrolledWindow::builder()
        .hscrollbar_policy(PolicyType::Never)
        .width_request(300)
        .child(&list_box)
        .build();

    let layout_box = gtk::Box::builder()
        .orientation(Orientation::Horizontal)
        .build();

    layout_box.append(&scrolled_window);

    let window = ApplicationWindow::builder()
        .title("Saboru")
        .width_request(1024)
        .height_request(768)
        .child(&layout_box)
        .application(app)
        .build();
    window.present();
}
