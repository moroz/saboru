use std::cell::Cell;
use std::rc::Rc;
use std::sync::OnceLock;
use std::thread;
use std::time::Duration;

use ashpd::desktop::account::UserInformation;
use ashpd::WindowIdentifier;
use gtk::builders::ButtonBuilder;
use gtk::gio;
use gtk::glib::{self, clone, closure_local};
use gtk::{prelude::*, ApplicationWindow, Box, Button, Orientation};
use gtk::{Application, Label};
use tokio::runtime::Runtime;

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

fn runtime() -> &'static Runtime {
    static RUNTIME: OnceLock<Runtime> = OnceLock::new();
    RUNTIME.get_or_init(|| Runtime::new().expect("Setting up tokio runtime failed."))
}

fn build_ui_with_custom_button(app: &Application) {
    let button = Button::builder()
        .label("Click me")
        .margin_end(12)
        .margin_start(12)
        .margin_bottom(12)
        .margin_top(12)
        .build();

    let (sender, receiver) = async_channel::bounded(1);

    button.connect_clicked(move |_| {
        runtime().spawn(clone!(@strong sender => async move {
            let response = reqwest::get("https://moroz.dev").await;
            sender.send(response).await.expect("The channel needs to be open.");
        }));
    });

    glib::spawn_future_local(async move {
        while let Ok(response) = receiver.recv().await {
            if let Ok(response) = response {
                println!("Status: {}", response.status());
            } else {
                println!("Could not make a `GET` request.");
            }
        }
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
