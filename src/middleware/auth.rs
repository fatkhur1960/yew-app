use yew_router::prelude::Route;

use super::Middleware;
pub struct AuthMiddleware;

impl Middleware for AuthMiddleware {
    fn before_enter(from: Route, to: Route, next: fn() -> Route) -> Route {
        todo!()
    }
}