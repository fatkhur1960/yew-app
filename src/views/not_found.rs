use yew::{prelude::*, utils::document};

pub struct NotFound {
    _link: ComponentLink<Self>,
}

pub enum Msg {}

impl Component for NotFound {
    type Message = Msg;
    type Properties = ();

    fn rendered(&mut self, _first_render: bool) {
        document().set_title("Page Not Found")
    }

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
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
            <main class="animate__animated animate__fadeIn">
                <div class="not-found text-center">
                    <h1>{"404"}</h1>
                    <h2>{"Page Not Found"}</h2>
                </div>
            </main>
        }
    }
}
