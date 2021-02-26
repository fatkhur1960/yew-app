use yew::{agent::Dispatcher, prelude::*, utils::document};

use crate::{JsonValue, utils::{RouteEvent, RouterAgent}};

use super::Props;

#[derive(Default)]
pub struct UrlParam {
    pub id: i32,
    pub slug: String,
}

pub struct HomeView {
    _link: ComponentLink<Self>,
    props: Props,
}

pub enum Msg {}

impl Component for HomeView {
    type Message = Msg;
    type Properties = Props;

    fn rendered(&mut self, _first_render: bool) {
        document().set_title("Home")
    }

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self { 
        Self {
            _link: link,
            props,
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
            <h1>{"Home Page"}</h1>
        }
    }
}
