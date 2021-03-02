use crate::routes::RouteHandler;
use crate::utils::{RouteEvent, RouterAgent};
use yew::prelude::*;
use yew_router::prelude::Route;

pub enum Msg {
    UpdateRoute(Route<bool>),
}

pub struct RouterView {
    current_route: Route<bool>,
    agent: Box<dyn Bridge<RouterAgent>>,
    handler: RouteHandler,
}

impl Component for RouterView {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let callback = link.callback(Msg::UpdateRoute);
        let mut agent = RouterAgent::bridge(callback);
        agent.send(RouteEvent::GetCurrentRoute);

        Self {
            agent,
            handler: RouteHandler::new(),
            current_route: Route {
                route: String::from("/"),
                state: true,
            },
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::UpdateRoute(route) => {
                self.current_route = route.clone();
                true
            }
        }
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        self.handler.render_view(self.current_route.clone())
    }
}
