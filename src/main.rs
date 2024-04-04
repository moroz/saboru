use std::cell::Cell;
use std::rc::Rc;

use gtk::builders::ButtonBuilder;
use gtk::glib::{self, clone, closure_local};
use gtk::{prelude::*, ApplicationWindow, Box, Button, Orientation};
use gtk::{Application, Label};

mod custom_button;

use custom_button::CustomButton;

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
    let button = CustomButton::with_label("Click me!");
    button.set_margin_top(12);
    button.set_margin_bottom(12);
    button.set_margin_end(12);
    button.set_margin_start(12);

    button.connect_closure(
        "max-number-reached",
        false,
        closure_local!(move |_button: CustomButton, number: i32| {
            println!("The maximum number {} has been reached", number);
        }),
    );
    let window = ApplicationWindow::builder()
        .application(app)
        .title("CustomButton demo")
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
