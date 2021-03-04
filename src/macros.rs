#![allow(unused_macros)]
macro_rules! redirect {
    ($target:expr) => {
        use crate::routes::ROUTE_HANDLER;
        use crate::routes::{RouteEvent::ChangeRoute, RouterAgent};
        use yew::agent::Dispatched;
        use yew_router::prelude::Route;

        let mut router = RouterAgent::dispatcher();
        let state = ROUTE_HANDLER.get_route_state($target);
        router.send(ChangeRoute(Route {
            route: $target.to_string(),
            state,
        }));
    };
}

macro_rules! console_log {
    ($($t:tt)*) => (crate::utils::bindings::log(&format_args!($($t)*).to_string()))
}

macro_rules! console_error {
    ($($t:tt)*) => (crate::utils::bindings::error(&format_args!($($t)*).to_string()))
}
