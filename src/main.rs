use std::cell::Cell;
use std::rc::Rc;

use gtk::builders::ButtonBuilder;
use gtk::glib::{self, clone};
use gtk::{gio, ListView, SingleSelection};
use gtk::{
    prelude::*, ApplicationWindow, Box, Button, ListBox, ListItem, Orientation, ScrolledWindow,
    SignalListItemFactory,
};
use gtk::{Application, Label};
use integer_object::IntegerObject;

const APP_ID: &str = "org.gtk_rs.HelloWorld1";

mod integer_object;

fn main() -> glib::ExitCode {
    let app = Application::builder().application_id(APP_ID).build();
    app.connect_activate(build_list);
    app.run()
}

fn button() -> ButtonBuilder {
    Button::builder()
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
}

fn build_list(app: &Application) {
    let vector: Vec<IntegerObject> = (0..=100_000).map(IntegerObject::new).collect();
    let model = gio::ListStore::new::<IntegerObject>();
    model.extend_from_slice(&vector);

    let factory = SignalListItemFactory::new();
    factory.connect_setup(move |_, list_item| {
        let label = Label::new(None);
        list_item
            .downcast_ref::<ListItem>()
            .expect("Needs to be ListItem")
            .set_child(Some(&label));
    });

    factory.connect_bind(move |_, list_item| {
        let integer_object = list_item
            .downcast_ref::<ListItem>()
            .expect("Needs to be ListItem")
            .item()
            .and_downcast::<IntegerObject>()
            .expect("The item has to be an IntegerObject.");

        let label = list_item
            .downcast_ref::<ListItem>()
            .expect("Needs to be ListItem")
            .child()
            .and_downcast::<Label>()
            .expect("The child has to be a Label.");

        label.set_label(&integer_object.number().to_string());
    });

    let selection_model = SingleSelection::new(Some(model));
    let list_view = ListView::new(Some(selection_model), Some(factory));

    let scrolled_window = ScrolledWindow::builder()
        .hscrollbar_policy(gtk::PolicyType::Never)
        .min_content_width(360)
        .child(&list_view)
        .build();

    let window = ApplicationWindow::builder()
        .application(app)
        .title("My GTK App")
        .default_width(600)
        .default_height(300)
        .child(&scrolled_window)
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
