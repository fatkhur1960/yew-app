use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};
use yew::{html, Html};

#[derive(Debug, Clone)]
pub struct Route<'a> {
    pub path: &'a str,
    pub view: Html,
}

impl<'a> Route<'a> {
    pub fn new(path: &'a str, view: Html) -> Self {
        Route { path, view }
    }
}

type RoutesMap = Arc<RwLock<HashMap<&'static str, Route<'static>>>>;

#[derive(Debug, Clone)]
pub struct AppRoute {
    routes: RoutesMap,
}

impl AppRoute {
    pub fn new() -> Self {
        AppRoute {
            routes: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub fn register(&mut self, route: Route<'static>) -> &mut AppRoute {
        if let Ok(mut routes) = self.routes.write() {
            routes.insert(route.path, route);
        }
        self
    }

    pub fn is_available(&self) -> bool {
        self.routes.read().unwrap().len() > 0
    }

    pub fn get_keys(&self) -> Option<Vec<String>> {
        match self.routes.read() {
            Ok(routes) => {
                let keys = routes
                    .keys()
                    .into_iter()
                    .map(|key| key.to_string())
                    .collect();
                Some(keys)
            }
            Err(_) => None,
        }
    }

    pub fn get_view(&self, path: &str) -> Result<Html, Html> {
        match self.routes.read() {
            Ok(routes) => match routes.get(path) {
                Some(route) => Ok(route.view.clone()),
                None => Err(html! {
                    <h1>{"Page not found"}</h1>
                }),
            },
            Err(e) => Err(html! {
                <div>
                    <h1>{"Internal Error"}</h1>
                    { e.to_string() }
                </div>
            }),
        }
    }
}
