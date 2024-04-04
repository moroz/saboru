use std::cell::Cell;

use glib::Properties;
use gtk::glib;
use gtk::prelude::*;
use gtk::subclass::prelude::*;

#[derive(Properties, Default)]
#[properties(wrapper_type = super::IntegerObject)]
pub struct IntegerObject {
    #[property(get, set)]
    number: Cell<i32>,
}

#[glib::object_subclass]
impl ObjectSubclass for IntegerObject {
    const NAME: &'static str = "MyGtkAppIntegerObject";
    type Type = super::IntegerObject;
}

#[glib::derived_properties]
impl ObjectImpl for IntegerObject {}
