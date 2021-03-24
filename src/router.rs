//! You can register routes here.
//! Note: The route definition need exact name of Component View
use crate::routes::{AppRouteHandler, AppRouteState};
use crate::{utils::route_parser, views};
use yew::{html, Html};
use yew_router::matcher::{MatcherSettings, RouteMatcher};
use yew_router::prelude::{Route, Switch};

#[derive(RouteDerive, Clone, Debug, PartialEq)]
pub enum AppRoute {
    #[params(path = "/", view = "LoginView", navbar = false, sidebar = false)]
    Login,
    #[params(
        path = "/register",
        view = "RegisterView",
        navbar = false,
        sidebar = false
    )]
    Register,
    #[params(
        path = "/reset-password",
        view = "RegisterView",
        navbar = false,
        sidebar = false
    )]
    ResetPassword,
    #[params(path = "/dashboard{*:inner}", view = "DashboardView", auth = true)]
    Dashboard(String)
}

#[derive(Switch, RouteChild, Clone, Debug, PartialEq)]
pub enum DashboardRoute {
    #[to = ""]
    #[view(DashboardMain)]
    None,
    #[to = "/tasks"]
    #[view(TaskView)]
    Task,
    #[to = "/projects"]
    #[view(ProjectView)]
    Project,
    #[to = "/todos"]
    #[view(TodoView)]
    Todo,
}
