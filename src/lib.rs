#![recursion_limit="256"]
#![allow(unused_imports, unused_variables, dead_code)]
extern crate yew;
extern crate wasm_bindgen;

#[macro_use]
extern crate proc_macros;

use wasm_bindgen::prelude::*;
use yew::prelude::*;

#[macro_use]
mod macros;
#[macro_use]
mod console;
mod app;
mod utils;
mod router;
mod components;
mod views;

pub type JsonValue = serde_json::Value;

pub use self::{components::*, router::AppRoute, app::RegisteredRoutes};


#[wasm_bindgen(start)]
pub fn run_app() {
    env_logger::init();

    App::<app::AppView>::new().mount_to_body();
}
