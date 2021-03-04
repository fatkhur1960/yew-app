#![recursion_limit = "256"]
#![allow(unused_variables, dead_code)]
extern crate wasm_bindgen;
extern crate yew;

#[macro_use]
extern crate proc_macros;
#[macro_use]
extern crate lazy_static;
extern crate validator;
extern crate yewtil;
extern crate wee_alloc;

// Use `wee_alloc` as the global allocator.
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

use wasm_bindgen::prelude::*;
use yew::{prelude::*, utils::document};

pub type JsonValue = serde_json::Value;

#[macro_use]
mod macros;
mod app;
mod components;
mod models;
mod router;
mod routes;
mod service;
mod utils;
mod views;

pub use self::components::*;
pub use router::AppRoute;

lazy_static! {
    static ref PUBLIC_URL: &'static str = "https://api.racta.dev.ansvia.com";
    static ref PRIVATE_URL: &'static str = "https://api.racta.dev.ansvia.com/private";
}

#[wasm_bindgen]
pub fn run_app() {
    if let Some(loading) = document().get_element_by_id("loading") {
        loading.set_class_name("animate__animated animate__fadeOut");
    }
    App::<app::AppView>::new().mount_to_body();
}
