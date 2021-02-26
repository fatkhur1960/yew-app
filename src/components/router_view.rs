use crate::{
    utils::{RouteEvent, RouteHandler, RouterAgent},
    AppRoute,
};
use yew::prelude::*;

pub enum Msg {
    UpdateRoute(String),
}

pub struct RouterView {
    handler: RouteHandler,
    current_route: String,
    _agent: Box<dyn Bridge<RouterAgent>>,
}

impl Component for RouterView {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let callback = link.callback(Msg::UpdateRoute);
        let mut agent = RouterAgent::bridge(callback);
        agent.send(RouteEvent::GetCurrentRoute);

        Self {
            handler: RouteHandler::new(),
            current_route: String::new(),
            _agent: agent,
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
        false
    }

    fn view(&self) -> Html {
        self.handler.render_view(self.current_route.clone())
    }
}
