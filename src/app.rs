use crate::{
    components::{Notification, RouterLink},
    routes::{AppRouteState, RouterAgent},
    utils::token,
};
use crate::{router_view::RouterView, routes::RouteEvent, AppRoute};
use yew::prelude::*;
use yew_router::prelude::Route;

pub struct AppView {
    state: AppRouteState,
    agent: Box<dyn Bridge<RouterAgent>>,
}

pub enum Msg {
    UpdateRoute(Route<AppRouteState>),
}

impl Component for AppView {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let callback = link.callback(Msg::UpdateRoute);
        let mut agent = RouterAgent::bridge(callback);
        agent.send(RouteEvent::GetCurrentRoute);

        Self {
            agent,
            state: AppRouteState {
                auth: true,
                navbar: true,
                sidebar: true,
            },
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::UpdateRoute(route) => {
                let next = route.to_string();
                self.state = route.state;

                token::auth_middleware(self.state.auth, "/login", Some(next));

                true
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div class="wrapper">
                <Notification />
                {
                    if self.state.navbar {
                        html! {
                            <navbar class="navbar navbar-expand-md fixed-top navbar-dark bg-dark">
                                <div class="container">
                                    <RouterLink class="navbar-brand" to=AppRoute::Dashboard>{"Cover"}</RouterLink>
                                    <div class="navbar-collapse offcanvas-collapse" id="navbarsExampleDefault">
                                        <ul class="navbar-nav ml-auto">
                                            <li class="nav-item">
                                                <RouterLink class="nav-link" to=AppRoute::Dashboard>{"Home"}</RouterLink>
                                            </li>
                                        </ul>
                                    </div>
                                </div>
                            </navbar>
                        }
                    } else {
                        html! {}
                    }
                }
                <RouterView default_route=AppRoute::Dashboard/>
            </div>
        }
    }
}
