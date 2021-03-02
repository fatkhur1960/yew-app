#![recursion_limit = "256"]
#![allow(unused_imports, unused_variables, dead_code)]
extern crate wasm_bindgen;
extern crate yew;

#[macro_use]
extern crate proc_macros;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;
extern crate web_logger;
extern crate validator;
extern crate yewtil;

use wasm_bindgen::prelude::*;
use web_logger::Config;
use yew::prelude::*;

pub type JsonValue = serde_json::Value;

#[macro_use]
mod macros;
mod app;
mod components;
mod routes;
mod service;
mod utils;
mod views;
mod models;
mod middleware;

pub use self::components::*;

lazy_static! {
    static ref PUBLIC_URL: &'static str = "https://api.racta.dev.ansvia.com";
    static ref PRIVATE_URL: &'static str = "https://api.racta.dev.ansvia.com/private";
}

#[wasm_bindgen(start)]
pub fn run_app() {
    App::<app::AppView>::new().mount_to_body();
}
