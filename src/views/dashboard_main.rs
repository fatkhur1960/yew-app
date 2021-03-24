use yew::prelude::*;

pub struct DashboardMain {
    _link: ComponentLink<Self>,
}

pub enum Msg {}

impl Component for DashboardMain {
    type Message = Msg;
    type Properties = ();

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
            <h1>{"Dashboard Content"}</h1>
        }
    }
}
