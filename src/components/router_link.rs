use crate::utils::{RouteEvent, RouterAgent};
use crate::{utils::route_parser, JsonValue};
use yew::events::MouseEvent;
use yew::virtual_dom::VNode;
use yew::{agent::Dispatcher, prelude::*};
use yew_router::prelude::Route;

pub struct RouterLink {
    props: Props,
    link: ComponentLink<Self>,
    router: Dispatcher<RouterAgent>,
}

#[derive(Properties, Clone, Default, Debug)]
pub struct Props {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub class: String,
    #[prop_or_default]
    pub to: String,
    #[prop_or_default]
    pub exact: bool,
}

pub enum Msg {
    Clicked,
}

impl Component for RouterLink {
    type Properties = Props;
    type Message = Msg;

    fn view(&self) -> Html {
        let target: &str = self.props.to.as_str();
        let class = self.props.class.clone();
        let cb = self.link.callback(|event: MouseEvent| {
            event.prevent_default();
            Msg::Clicked
        });

        html! {
            <a class=class onclick=cb href=target>
                { self.props.children.iter().collect::<VNode>() }
            </a>
        }
    }

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            props,
            router: RouterAgent::dispatcher(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Clicked => {
                self.router.send(RouteEvent::ChangeRoute(Route {
                    route: self.props.to.clone(),
                    state: self.props.exact,
                }));
                true
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }
}
