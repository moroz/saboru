mod imp;

use gtk::{glib, prelude::*, subclass::prelude::*};

glib::wrapper! {
    pub struct SidebarRow(ObjectSubclass<imp::SidebarRow>)
    @extends gtk::Widget, gtk::Box;
}

impl Default for SidebarRow {
    fn default() -> Self {
        glib::Object::new()
    }
}

impl SidebarRow {
    pub fn set_content(&self, label: String) {
        let imp = self.imp();
        imp.name.set_text(&label);
    }
}
