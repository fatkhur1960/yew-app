use crate::{components::RouterLink, AppRoute};
use yew::{prelude::*, utils::document};

pub struct DashboardView {
    link: ComponentLink<Self>,
}

pub enum Msg {}

impl Component for DashboardView {
    type Message = Msg;
    type Properties = ();

    fn rendered(&mut self, first_render: bool) {
        document().set_title("Racta - Dashboard");
    }

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <main role="main" class="container mt-5 animate__animated animate__fadeIn">
                <RouterLink to=AppRoute::Project("test-project".to_string())>{"Test Project"}</RouterLink>
            </main>
        }
    }
}
