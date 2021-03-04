use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use yew_router::matcher::RouteMatcher;

mod router_agent;
pub use router_agent::{RouteEvent, RouterAgent};

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct AppRouteState {
    pub auth: bool,
    pub navbar: bool,
    pub sidebar: bool,
}

#[derive(Debug, Clone)]
pub struct AppRouteHandler {
    pub routes: Vec<RouteMatcher>,
    pub meta: Box<HashMap<String, AppRouteState>>,
}

lazy_static! {
    pub static ref ROUTE_HANDLER: AppRouteHandler = AppRouteHandler::new();
}