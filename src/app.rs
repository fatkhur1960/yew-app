use crate::router::AppRoute;
use crate::router_view::RouterView;
use crate::{
    components::RouterLink,
    views::{AboutView, HomeView},
};
use yew::prelude::*;

register_routes![
    route("/", HomeView),
    route("/about", AboutView)
];

pub struct AppView {}

impl Component for AppView {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div class="wrapper">
                <RouterLink to="/">{"Home"}</RouterLink>{" | "}
                <RouterLink to="/blog">{"Blog"}</RouterLink>{" | "}
                <RouterLink to="/about">{"About"}</RouterLink>{" | "}
                <RouterLink to="/about/10">{"Contact"}</RouterLink>
                <hr/>
                <RouterView/>
            </div>
        }
    }
}
