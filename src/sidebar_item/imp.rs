use glib::Properties;
use gtk::glib;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use std::cell::RefCell;

use super::SidebarItemData;

#[derive(Properties, Default)]
#[properties(wrapper_type = super::SidebarItem)]
pub struct SidebarItem {
    #[property(name = "label", get, set, type = String, member = label)]
    pub data: RefCell<SidebarItemData>,
}

#[glib::object_subclass]
impl ObjectSubclass for SidebarItem {
    const NAME: &'static str = "SidebarItem";
    type Type = super::SidebarItem;
}

#[glib::derived_properties]
impl ObjectImpl for SidebarItem {}
