use std::cell::Cell;
use std::rc::Rc;
use std::thread;
use std::time::Duration;

use gtk::builders::ButtonBuilder;
use gtk::gio;
use gtk::glib::{self, clone, closure_local};
use gtk::{prelude::*, ApplicationWindow, Box, Button, Orientation};
use gtk::{Application, Label};

const APP_ID: &str = "org.gtk_rs.HelloWorld1";

fn main() -> glib::ExitCode {
    let app = Application::builder().application_id(APP_ID).build();
    app.connect_activate(build_ui_with_custom_button);
    app.run()
}

fn button() -> ButtonBuilder {
    Button::builder()
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
}

fn build_ui_with_custom_button(app: &Application) {
    let button = Button::builder()
        .label("Click me")
        .margin_end(12)
        .margin_start(12)
        .margin_bottom(12)
        .margin_top(12)
        .build();

    button.connect_clicked(move |button| {
        glib::spawn_future_local(clone!(@weak button => async move {
            button.set_sensitive(false);
            glib::timeout_future_seconds(5).await;
            button.set_sensitive(true);
        }));
    });

    let window = ApplicationWindow::builder()
        .application(app)
        .title("My GTK App")
        .child(&button)
        .build();
    window.present();
}

fn build_ui(app: &Application) {
    let button_increase = button().label("Increase").build();
    let button_decrease = button().label("Decrease").build();
    let label = Label::builder()
        .margin_end(12)
        .margin_start(12)
        .margin_bottom(12)
        .margin_top(12)
        .build();

    let number = Rc::new(Cell::new(0));

    label.set_label(&number.get().to_string());

    button_increase.connect_clicked(clone!(@weak number, @weak label => move |_| {
        number.set(number.get() + 1);
        label.set_label(&number.get().to_string());
    }));
    button_decrease.connect_clicked(clone!(@weak label => move |_| {
        number.set(number.get() - 1);
        label.set_label(&number.get().to_string());
    }));

    let layout_box = Box::builder().orientation(Orientation::Horizontal).build();
    layout_box.append(&button_decrease);
    layout_box.append(&label);
    layout_box.append(&button_increase);

    let window = ApplicationWindow::builder()
        .application(app)
        .default_width(800)
        .default_height(640)
        .title("My GTK App")
        .child(&layout_box)
        .build();

    window.present();
}
