use crate::{
    components::{Sidebar, SidebarItem},
    models::Account,
    service::AccountService,
    utils::notif_agent::{NotifAgent, NotifEvent},
    views::RenderView,
    AppRoute, DashboardRoute as DRoute, Notify,
};
use serde::Deserialize;
use yew::{prelude::*, utils::document};
use yew_router::prelude::Route;
use yewtil::future::LinkFuture;

#[derive(Debug, Clone, Params, Deserialize, Default)]
pub struct Params {
    pub inner: String,
}

#[derive(Clone)]
pub struct DashboardView {
    link: ComponentLink<Self>,
    current_route: Route,
    params: Params,
    title: String,
    account_service: AccountService,
    account: Option<Account>,
}
pub enum Msg {
    SuccessResp(Account),
    ErrorResp(String),
    UpdateRoute(Route, String),
}

impl Component for DashboardView {
    type Message = Msg;
    type Properties = ();

    fn rendered(&mut self, first_render: bool) {
        document().set_title(&format!("Racta - {}", self.title));

        if first_render {
            let mut account_service = self.account_service.clone();
            self.link.send_future(async move {
                match account_service.get_profile().await {
                    Ok(res) => {
                        if res.code == 0 {
                            Msg::SuccessResp(res.result.unwrap())
                        } else {
                            Msg::ErrorResp(res.description)
                        }
                    }
                    Err(err) => Msg::ErrorResp(err.to_string()),
                }
            });
        }
    }

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let params = Params::new();
        let path = params.clone().inner;
        let current_route = Route::from(path);

        Self {
            link,
            current_route,
            params,
            title: "Dashboard".into(),
            account_service: AccountService::new(),
            account: None,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        let mut notif_agent = NotifAgent::dispatcher();
        match msg {
            Msg::UpdateRoute(route, title) => {
                self.current_route = route;
                self.title = title;
            }
            Msg::SuccessResp(account) => self.account = Some(account),
            Msg::ErrorResp(msg) => notif_agent.send(NotifEvent::Push(Notify::error(msg))),
        }

        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let cb: Callback<(Route, String)> = self
            .link
            .callback(|(route, title)| Msg::UpdateRoute(route, title));
        let current = self.current_route.clone();
        html! {
            <main class="main-container animate__animated animate__fadeIn">
                <div class="col-md-12">
                    <div class="row">
                        <div class="col-md-3">
                            <Sidebar<DRoute> current=current cb=cb base=AppRoute::Dashboard("".into())
                            account=self.account.clone()
                            menus=vec![
                                SidebarItem::new("Dashboard", "view-dashboard", DRoute::None),
                                SidebarItem::new("Project", "file-tree", DRoute::Project),
                                SidebarItem::new("Task", "briefcase-edit-outline", DRoute::Task),
                                SidebarItem::new("Todo", "format-list-bulleted", DRoute::Todo),
                            ]/>
                        </div>
                        <div class="col-md-9">
                        { DRoute::render_view(self.current_route.clone(), html! {<h1>{"Not Found"}</h1>}) }
                        </div>
                    </div>
                </div>
            </main>
        }
    }
}
