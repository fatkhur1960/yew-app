//! You can register routes here.
//! Note: The route definition need exact name of Component View
use crate::routes::{AppRouteHandler, AppRouteState};
use crate::{utils::route_parser, views};
use yew::{html, Html};
use yew_router::matcher::{MatcherSettings, RouteMatcher};
use yew_router::prelude::Route;

#[derive(RouteDerive, Clone, Debug)]
pub enum AppRoute {
    #[params(path = "/login", view = "LoginView", navbar = false, sidebar = false)]
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
    #[params(path = "/", view = "DashboardView", auth = true)]
    Dashboard,
    #[params(path = "/project/{name}", view = "ContactView", auth = true)]
    Project(String),
}