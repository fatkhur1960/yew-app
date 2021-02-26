use yew::{prelude::*, utils::document};

use super::Props;

pub struct AboutView {
    _link: ComponentLink<Self>,
    props: Props,
}

pub enum Msg {}

impl Component for AboutView {
    type Message = Msg;
    type Properties = Props;

    fn rendered(&mut self, _first_render: bool) {
        document().set_title("About")
    }

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { _link: link, props }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <h1>{"About Page"}</h1>
        }
    }
}
