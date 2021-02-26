mod home_view;
mod about_view;
pub use self::{home_view::HomeView, about_view::AboutView};

use yew::prelude::*;
use crate::JsonValue;

#[derive(Properties, Clone, Default, Debug)]
pub struct Props {
    #[prop_or_default]
    pub params: JsonValue,
    #[prop_or_default]
    pub query: JsonValue,
}