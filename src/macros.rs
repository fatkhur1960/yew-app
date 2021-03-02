#![allow(unused_macros)]

/// Usage: register_routes[route(path, component_view)]
macro_rules! register_routes {
    ([ $(route($path:expr, $view:ident)),* ]) => {
        use proc_macros::RouteHolder;
        use std::collections::HashMap;
        use crate::{views, utils::route_parser};
        use yew_router::prelude::Route;
        use yew_router::matcher::{MatcherSettings, RouteMatcher};
        use yew::{Html, html};

        lazy_static! {
            static ref ROUTES: Vec<RouteMatcher> = {
                let mut routes = Vec::new();
                let setting = MatcherSettings::default();
                add_route!(routes, setting, [$($path),*]);
                routes
            };
        }

        #[derive(RouteHolder, Debug, Clone)]
        #[routes($($view=$path,)*)]
        pub struct RouteHandler {
            pub routes: Vec<RouteMatcher>,
        }
    };
}

macro_rules! add_route {
    ($varname:ident, $setting:ident, [$($path:expr),*]) => {
        $($varname.push(RouteMatcher::new($path, $setting).expect("invalid path"));)*
    };
}

macro_rules! console_log {
    ($($t:tt)*) => (crate::utils::bindings::log(&format_args!($($t)*).to_string()))
}

macro_rules! console_error {
    ($($t:tt)*) => (crate::utils::bindings::error(&format_args!($($t)*).to_string()))
}