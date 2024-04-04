mod imp;

use glib::Object;
use gtk::glib;
use gtk::subclass::prelude::*;

glib::wrapper! {
    pub struct SidebarItem(ObjectSubclass<imp::SidebarItem>);
}

impl SidebarItem {
    pub fn new(label: String) -> Self {
        Object::builder().property("label", label).build()
    }
}

#[derive(Default, Clone)]
pub struct SidebarItemData {
    pub label: String,
}
