mod dashboard;
mod dashboard_main;
mod login;
mod not_found;
mod project;
mod register;
mod task;
mod todo;

use yew::Html;
use yew_router::prelude::Route;

pub use self::{
    dashboard::DashboardView, dashboard_main::DashboardMain, login::LoginView, not_found::NotFound,
    project::ProjectView, register::RegisterView, task::TaskView, todo::TodoView,
};

pub trait RenderView {
    fn render_view(route: Route, not_found: Html) -> Html;
}