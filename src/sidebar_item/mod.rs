mod imp;

use glib::Object;
use gtk::glib;

glib::wrapper! {
    pub struct SidebarItem(ObjectSubclass<imp::SidebarItem>);
}

impl SidebarItem {
    pub fn new(label: String, id: impl Into<i64>) -> Self {
        Object::builder()
            .property("label", label)
            .property("id", id.into())
            .build()
    }
}

#[derive(Default, Clone, Debug)]
pub struct SidebarItemData {
    pub label: String,
    pub id: i64,
}
