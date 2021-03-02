use serde::{Deserialize, Serialize};
use yew::{prelude::*, utils::document};

use crate::{JsonValue, utils::route_parser::from_str};

use super::Props;

#[derive(Default, Serialize, Deserialize)]
struct Params {
    pub name: String,
}

pub struct ContactView {
    _link: ComponentLink<Self>,
    props: Props,
    params: Params,
}

pub enum Msg {}

impl Component for ContactView {
    type Message = Msg;
    type Properties = Props;

    fn rendered(&mut self, _first_render: bool) {
        document().set_title(&format!("Project - {}", self.params.name))
    }

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let params = serde_json::from_value(props.clone().params).unwrap_or_default();
        Self {
            _link: link,
            props,
            params,
        }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <h1>{"Project: "} { self.params.name.clone() } </h1>
        }
    }
}
