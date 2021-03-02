use yew_router::prelude::Route;

pub mod auth;

pub trait Middleware {
    fn before_enter(from: Route, to: Route, next: fn() -> Route) -> Route;
}