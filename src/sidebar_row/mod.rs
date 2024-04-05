mod imp;

use gtk::{gdk::Cursor, glib, prelude::*, subclass::prelude::*};

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
        self.set_cursor(Cursor::from_name("pointer", None).as_ref());
        let imp = self.imp();
        imp.name.set_text(&label);
    }
}
