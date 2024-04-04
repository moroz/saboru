use gtk::{glib, subclass::prelude::*};

#[derive(Default, gtk::CompositeTemplate)]
#[template(file = "sidebar_row.ui")]
pub struct SidebarRow {
    #[template_child]
    pub name: TemplateChild<gtk::Label>,
    #[template_child]
    pub avatar: TemplateChild<gtk::Image>,
}

#[glib::object_subclass]
impl ObjectSubclass for SidebarRow {
    const NAME: &'static str = "SidebarRow";
    type Type = super::SidebarRow;
    type ParentType = gtk::Box;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
    }

    fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
        obj.init_template();
    }
}

impl ObjectImpl for SidebarRow {}
impl WidgetImpl for SidebarRow {}
impl BoxImpl for SidebarRow {}
