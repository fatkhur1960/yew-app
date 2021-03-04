use crate::routes::{AppRouteHandler, RouteEvent, RouterAgent};
use crate::{
    routes::{AppRouteState, ROUTE_HANDLER},
    AppRoute,
};
use yew::prelude::*;
use yew_router::prelude::Route;

pub enum Msg {
    UpdateRoute(Route<AppRouteState>),
}

pub struct RouterView {
    current_route: Route<AppRouteState>,
    agent: Box<dyn Bridge<RouterAgent>>,
    handler: AppRouteHandler,
}

#[derive(Properties, Clone, Debug)]
pub struct Props {
    pub default_route: AppRoute,
}

impl Component for RouterView {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let callback = link.callback(Msg::UpdateRoute);
        let mut agent = RouterAgent::bridge(callback);
        agent.send(RouteEvent::GetCurrentRoute);

        Self {
            agent,
            handler: ROUTE_HANDLER.clone(),
            current_route: props.default_route.into(),
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
        let route = self.current_route.clone();
        self.handler.render_view(route)
    }
}
