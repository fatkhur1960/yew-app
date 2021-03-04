use serde::{Deserialize, Serialize};
use yew::{prelude::*, utils::document};

#[derive(Params, Default, Serialize, Deserialize)]
struct NameParam {
    pub name: String,
}

pub struct ContactView {
    _link: ComponentLink<Self>,
    params: NameParam,
}

pub enum Msg {}

impl Component for ContactView {
    type Message = Msg;
    type Properties = ();

    fn rendered(&mut self, _first_render: bool) {
        document().set_title(&format!("Project - {}", self.params.name))
    }

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            _link: link,
            params: NameParam::new(),
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
