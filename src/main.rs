use gtk::gdk::Display;
use gtk::glib::clone;
use gtk::glib::{self};
use gtk::{gio, ApplicationWindow, CssProvider};
use gtk::{prelude::*, PolicyType, ScrolledWindow};
use gtk::{Application, Orientation};
use sidebar_item::SidebarItem;
use sidebar_row::SidebarRow;

use crate::sidebar_item::SidebarItemData;

const APP_ID: &str = "org.gtk_rs.todo";

mod sidebar_item;
mod sidebar_row;

fn main() -> glib::ExitCode {
    let app = Application::builder().application_id(APP_ID).build();
    app.connect_startup(|_| load_css());
    app.connect_activate(build_ui);
    app.run()
}

fn load_css() {
    let provider = CssProvider::new();
    provider.load_from_data(include_str!("css/style.css"));

    gtk::style_context_add_provider_for_display(
        &Display::default().unwrap(),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}

fn build_ui(app: &Application) {
    let contacts: Vec<&str> = vec!["Alice", "Bob"];
    let model = gio::ListStore::new::<SidebarItem>();
    for (index, name) in contacts.iter().enumerate() {
        model.append(&SidebarItem::new(name.to_string(), (index + 1) as i64));
    }

    let factory = gtk::SignalListItemFactory::new();
    factory.connect_setup(move |_factory, item| {
        let item = item.downcast_ref::<gtk::ListItem>().unwrap();
        let row = SidebarRow::default();
        item.set_child(Some(&row));
    });

    factory.connect_bind(move |_factory, item| {
        let item = item.downcast_ref::<gtk::ListItem>().unwrap();
        let sidebar_item = item.item().and_downcast::<SidebarItem>().unwrap();

        let child = item.child().and_downcast::<SidebarRow>().unwrap();
        child.set_content(sidebar_item.label());
    });

    let selection_model = gtk::SingleSelection::new(Some(model));

    selection_model.connect_selection_changed(move |selection, _, _| {
        if let Some(selected_item) = selection.selected_item() {
            let item = selected_item.downcast_ref::<SidebarItem>().unwrap();
            println!("{:?}", item.property::<i64>("id"));
        }
    });

    let list_view = gtk::ListView::new(Some(selection_model), Some(factory));

    let scrolled_window = ScrolledWindow::builder()
        .hscrollbar_policy(PolicyType::Never)
        .width_request(300)
        .css_classes(["sidebar"])
        .child(&list_view)
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
