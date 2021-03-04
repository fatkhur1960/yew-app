use std::collections::HashSet;
use yew::worker::*;
use yew_router::prelude::{Route, RouteService};

use crate::routes::AppRouteState;

use super::ROUTE_HANDLER;

/// Internal Message used for the RouteAgent.
#[derive(Debug)]
pub enum Msg {
    /// Message for when the route is changed.
    BrowserNavigationRouteChanged(Route<AppRouteState>), // TODO make this a route?
}

#[allow(dead_code)]
pub enum RouteEvent {
    /// Replaces the most recent Route with a new one and alerts connected components to the route
    /// change.
    ReplaceRoute(Route<AppRouteState>),
    /// Replaces the most recent Route with a new one, but does not alert connected components to
    /// the route change.
    ReplaceRouteNoBroadcast(Route<AppRouteState>),
    /// Changes the route using a Route struct and alerts connected components to the route change.
    ChangeRoute(Route<AppRouteState>),
    /// Changes the route using a Route struct, but does not alert connected components to the
    /// route change.
    ChangeRouteNoBroadcast(Route<AppRouteState>),
    /// Gets the current route.
    GetCurrentRoute,
}

pub struct RouterAgent {
    link: AgentLink<RouterAgent>,
    subscribers: HashSet<HandlerId>,
    route_service: RouteService<AppRouteState>,
}

impl Agent for RouterAgent {
    type Reach = Context<Self>;
    type Message = Msg;
    type Input = RouteEvent;
    type Output = Route<AppRouteState>;

    fn create(link: AgentLink<RouterAgent>) -> Self {
        let callback = link.callback(Msg::BrowserNavigationRouteChanged);
        let mut route_service = RouteService::new();
        route_service.register_callback(callback);

        Self {
            link,
            route_service,
            subscribers: HashSet::new(),
        }
    }

    fn update(&mut self, msg: Self::Message) {
        match msg {
            Msg::BrowserNavigationRouteChanged(route) => {
                for sub in &self.subscribers {
                    self.link.respond(*sub, route.clone());
                }
            }
        }
    }

    fn handle_input(&mut self, msg: Self::Input, who: HandlerId) {
        match msg {
            RouteEvent::ReplaceRoute(route) => {
                self.route_service
                    .replace_route(&route, route.state.clone());
                let route = self.route_service.get_route();
                for sub in &self.subscribers {
                    self.link.respond(*sub, route.clone());
                }
            }
            RouteEvent::ReplaceRouteNoBroadcast(route) => {
                self.route_service
                    .replace_route(&route, route.state.clone());
            }
            RouteEvent::ChangeRoute(route) => {
                // set the route
                self.route_service.set_route(&route, route.state.clone());
                // get the new route.
                let route = self.route_service.get_route();
                // broadcast it to all listening components
                for sub in &self.subscribers {
                    self.link.respond(*sub, route.clone());
                }
            }
            RouteEvent::ChangeRouteNoBroadcast(route) => {
                self.route_service.set_route(&route, route.state.clone());
            }
            RouteEvent::GetCurrentRoute => {
                let route = self.route_service.get_route().to_string();
                let state = ROUTE_HANDLER.get_route_state(&route);

                self.link.respond(who, Route {
                    route,
                    state,
                });
            }
        }
    }

    fn connected(&mut self, id: HandlerId) {
        self.subscribers.insert(id);
    }

    fn disconnected(&mut self, id: HandlerId) {
        self.subscribers.remove(&id);
    }
}
