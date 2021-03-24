use yew::prelude::*;
use yew_router::{prelude::RouteService, route::Route, Switch};

use crate::{models::Account, AppRoute};

#[derive(Clone)]
pub struct SidebarItem<SW>
where
    SW: Switch + Clone,
{
    pub title: String,
    pub icon: String,
    pub target: SW,
}

impl<SW> SidebarItem<SW>
where
    SW: Switch + Clone,
{
    pub fn new(title: &str, icon: &str, target: SW) -> Self {
        Self {
            title: title.to_string(),
            icon: icon.to_string(),
            target,
        }
    }
}

pub struct Sidebar<SW>
where
    SW: Switch + Clone + 'static,
{
    link: ComponentLink<Self>,
    props: Props<SW>,
    route_service: RouteService<()>,
}

#[derive(Properties, Clone)]
pub struct Props<SW>
where
    SW: Switch + Clone,
{
    pub cb: Callback<(Route, String)>,
    #[prop_or_default]
    pub menus: Vec<SidebarItem<SW>>,
    pub current: Route,
    #[prop_or(None)]
    pub base: Option<AppRoute>,
    #[prop_or(None)]
    pub account: Option<Account>,
}

pub enum Msg {
    Navigate((Route, String)),
}

impl<SW> Sidebar<SW>
where
    SW: Switch + Clone + 'static,
{
    fn extend_base(&self, end: String) -> String {
        let base = self.props.base.clone();
        if let Some(base) = base {
            format!("{}{}", base.clear(), end)
        } else {
            end
        }
    }
}

impl<SW> Component for Sidebar<SW>
where
    SW: Switch + Clone + 'static,
{
    type Message = Msg;
    type Properties = Props<SW>;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let route_service = RouteService::new();
        Sidebar {
            link,
            props,
            route_service,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        let base = self.props.base.clone();
        match msg {
            Msg::Navigate((route, title)) => {
                let target = self.extend_base(route.to_string());
                self.route_service.set_route(&target, route.state);
                self.props.cb.emit((route, title));
            }
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        let (profile_pic, full_name, email, role) = match self.props.account.clone() {
            Some(account) => (
                account.pic,
                account.full_name,
                account.email,
                account
                    .roles
                    .first()
                    .map(|r| r.to_owned())
                    .unwrap_or(String::new()),
            ),
            None => (String::new(), String::new(), String::new(), String::new()),
        };
        let base = &self.props.base;
        let current = self.props.current.clone();
        let onclick = |input: Route, title: &str| {
            let title = title.to_owned();
            self.link.callback(move |e: MouseEvent| {
                e.prevent_default();
                Msg::Navigate((input.clone(), title.clone()))
            })
        };
        html! {
            <div class="sidebar">
                <div class="sidebar-header">
                    <div class="d-flex">
                        <div class="image">
                            <img src=&profile_pic/>
                        </div>
                        <div class="ml-3 w-100">
                            <h4 class="mb-0 mt-0">{&full_name}</h4>
                            <span>{&role}</span>
                            <p>{&email}</p>
                        </div>
                    </div>
                </div>
                <div class="sidebar-menus">
                { self.props.menus.iter().map(|i| {
                    let target = i.target.clone();
                    let route = Route::from(target);
                    let href_target = self.extend_base(route.to_string());
                    let classes = if route == current {
                        "s-item active"
                    } else {
                        "s-item"
                    };

                    html! {
                        <a class=classes onclick=onclick(route, &i.title) href=href_target>
                            <i class=format!("mdi mdi-{}", i.icon)></i>{&i.title}
                        </a>
                    }
                }).collect::<Html>() }
                </div>
                <div class="sidebar-footer">
                    <a class="s-item red" href="#"><i class="mdi mdi-power"></i>{" Sign Out"}</a>
                </div>
            </div>
        }
    }
}
