use yew::prelude::*;

use crate::{AppRoute, RouterLink};

#[derive(Properties, PartialEq, Clone)]
pub struct Props {
    pub title: String,
    pub to: AppRoute,
    #[prop_or_default]
    pub icon: String,
}

pub struct NavItem(Props);

impl Component for NavItem {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        NavItem(props)
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        html! {
            <li class="nav-item">
                <RouterLink class="nav-link" to=self.0.to.clone()>
                    {&self.0.title}
                </RouterLink>
            </li>
        }
    }
}
