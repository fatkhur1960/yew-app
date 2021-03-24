pub mod forms;
mod nav_item;
mod navbar;
mod notification;
mod sidebar;
pub mod router_link;
pub mod router_view;

pub use self::{
    nav_item::NavItem, navbar::Navbar, notification::Notification, router_link::RouterLink,
    sidebar::{Sidebar, SidebarItem},
};

#[derive(Clone, PartialEq, Debug)]
pub struct Notify {
    pub title: String,
    pub description: String,
    pub n_type: String,
}

impl Notify {
    pub fn success(description: String) -> Self {
        Self {
            title: String::from("Success"),
            description,
            n_type: String::from("success"),
        }
    }

    pub fn info(description: String) -> Self {
        Self {
            title: String::from("Info"),
            description,
            n_type: String::from("info"),
        }
    }

    pub fn warning(description: String) -> Self {
        Self {
            title: String::from("Warning"),
            description,
            n_type: String::from("warning"),
        }
    }

    pub fn error(description: String) -> Self {
        Self {
            title: String::from("Error"),
            description,
            n_type: String::from("error"),
        }
    }
}
