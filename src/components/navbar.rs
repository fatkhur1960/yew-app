use crate::{NavItem, AppRoute, RouterLink};
use yew::prelude::*;
use yewtil::NeqAssign;

#[derive(Properties, PartialEq, Clone)]
pub struct Props {
    pub children: ChildrenWithProps<NavItem>,
    pub title: String,
    #[prop_or(true)]
    pub show: bool,
}

#[derive(PartialEq, Clone)]
pub struct Navbar(Props);

impl Component for Navbar {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Navbar(props)
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.0.neq_assign(props)
    }

    fn view(&self) -> Html {
        if self.0.show {
            html! {
                <navbar class="navbar navbar-expand-md fixed-top navbar-light">
                    <div class="nav-container">
                        <div class="col-md-12">
                            <div class="row">
                                <div class="col-md-3">
                                    <RouterLink class="navbar-brand" to=AppRoute::Dashboard("".into())>
                                        {&self.0.title}
                                    </RouterLink>
                                </div>
                                <div class="col-md-9">
                                    <div class="navbar-collapse offcanvas-collapse" id="navbarsExampleDefault">
                                        <ul class="navbar-nav ml-auto">
                                            { self.0.children.clone() }
                                        </ul>
                                    </div>
                                </div>
                            </div>
                        </div>
                    </div>
                </navbar>
            }
        } else {
            html! {}
        }
    }
}
