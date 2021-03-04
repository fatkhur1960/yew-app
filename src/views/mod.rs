mod not_found;
mod dashboard;
mod contact_view;
mod login;
mod register;

pub use self::{
    not_found::NotFound, dashboard::DashboardView, contact_view::ContactView, login::LoginView,
    register::RegisterView,
};
