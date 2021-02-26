/// Usage: register_routes[route(path, component_view)]
macro_rules! register_routes {
    [ $(route($path:expr, $view:ident)),* ] => {
        use crate::router::Route;
        use yew::html;
        use proc_macros::RouteHolder;

        #[derive(RouteHolder, Clone)]
        pub enum RegisteredRoutes {
            $(#[to(path = $path, view = $view)] $view,)*
        }

        impl AppRoute {
            pub fn init() -> AppRoute {
                let mut app_route = AppRoute::new();
                $(app_route.register(Route::new($path, html! {<$view/>}));)*

                app_route
            }
        }
    };
}
