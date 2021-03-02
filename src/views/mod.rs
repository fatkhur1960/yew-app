mod about_view;
mod blog_view;
mod contact_view;
mod home_view;
use std::sync::{Arc, Mutex};

pub use self::{
    about_view::AboutView, blog_view::BlogView, contact_view::ContactView, home_view::HomeView,
};

use crate::{JsonValue, utils::notif_agent::NotifAgent};
use yew::{agent::Dispatcher, prelude::*};

#[derive(Properties, PartialEq, Clone, Debug)]
pub struct Props {
    #[prop_or_default]
    pub params: JsonValue,
    #[prop_or_default]
    pub query: JsonValue,
}