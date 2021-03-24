use yew::{prelude::*, utils::document};

pub struct TodoView {
    _link: ComponentLink<Self>,
}

pub enum Msg {}

impl Component for TodoView {
    type Message = Msg;
    type Properties = ();

    fn rendered(&mut self, _first_render: bool) {
        document().set_title("Racta - Todo")
    }

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { _link: link }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <h1>{"Todo Page"}</h1>
        }
    }
}
