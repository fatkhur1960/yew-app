use crate::components::{Notification, Notify, RouterLink};
use crate::router_view::RouterView;
use yew::prelude::*;

pub struct AppView;

impl Component for AppView {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self { }
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let class = "nav-link";
        html! {
            <div class="wrapper">
                <Notification />
                // <navbar class="navbar navbar-expand-md fixed-top navbar-dark bg-dark">
                //     <div class="container">
                //         <RouterLink class="navbar-brand" to="/" exact=true>{"Cover"}</RouterLink>
                //         <div class="navbar-collapse offcanvas-collapse" id="navbarsExampleDefault">
                //             <ul class="navbar-nav ml-auto">
                //                 <li class="nav-item"><RouterLink class=class exact=true to="/">{"Home"}</RouterLink></li>
                //                 <li class="nav-item"><RouterLink class=class exact=true to="/projects">{"Projects"}</RouterLink></li>
                //                 <li class="nav-item"><RouterLink class=class exact=true to="/about">{"About"}</RouterLink></li>
                //                 <li class="nav-item"><RouterLink class=class to="/contact/10">{"Contact"}</RouterLink></li>
                //             </ul>
                //         </div>
                //     </div>
                // </navbar>
                <RouterView/>
            </div>
        }
    }
}
