use serde::{Deserialize, Serialize};
use yew::InputData;
use yew::{
    agent::{Dispatched, Dispatcher},
    prelude::*,
    utils::document,
    Component,
};
use yewtil::future::LinkFuture;

use crate::{
    forms::{Button, Field, Form},
    models::{AccessToken, Login},
    service::AuthService,
    utils::{
        notif_agent::{NotifAgent, NotifEvent},
        token,
    },
    AppRoute, Notify, RouterLink,
};

#[derive(Query, Debug, Clone, Serialize, Deserialize)]
struct NextQuery {
    pub next: Option<String>,
}

pub struct LoginView {
    link: ComponentLink<Self>,
    form: Form<Login>,
    notif_agent: Dispatcher<NotifAgent>,
    query: NextQuery,
}

pub enum Msg {
    Update,
    Submit,
    Authorized(AccessToken),
    Unauthorized(String),
}

impl Component for LoginView {
    type Message = Msg;
    type Properties = ();

    fn rendered(&mut self, first_render: bool) {
        document().set_title("Racta - Login");

        if first_render && token::is_authenticated() {
            redirect!("/");
        }
    }

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            query: NextQuery::new(),
            form: Form::new(Login {
                ..Default::default()
            }),
            notif_agent: NotifAgent::dispatcher(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Update => true,
            Msg::Submit => {
                if self.form.validate() {
                    let mut auth = AuthService::new();
                    let body = self.form.model();
                    self.form.processing(true);

                    self.link.send_future(async move {
                        match auth.authorize(&body).await {
                            Ok(res) => {
                                if res.code == 0 {
                                    Msg::Authorized(res.result.unwrap())
                                } else {
                                    Msg::Unauthorized(res.description)
                                }
                            }
                            Err(e) => Msg::Unauthorized(e.msg),
                        }
                    });
                }

                true
            }
            Msg::Authorized(at) => {
                token::set_token(Some(at.token));
                self.form.processing(false);
                let target = self.query.next.clone().unwrap_or("/".to_string());
                redirect!(&target);

                true
            }
            Msg::Unauthorized(msg) => {
                self.form.processing(false);
                self.notif_agent.send(NotifEvent::Push(Notify::error(msg)));

                true
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        html! {
            <main role="main" class="login animate__animated animate__fadeIn">
                <div class="login-container">
                    <div class="app-logo">
                        <img src="/images/racta.png"/>
                    </div>
                    <div class="login-form">
                        <h1 class="login-title">{"Login Your Account"}</h1>
                        <form>
                            <div class="form-group">
                                <label for="email">{"Email"}</label>
                                <Field<Login> input_type="text" form=&self.form name="email" oninput=self.link.callback(|_: InputData| Msg::Update) />
                                <div class=vec!["invalid-feedback", self.form.field_error("email")]>
                                    {&self.form.field_message("email")}
                                </div>
                            </div>
                            <div class="form-group">
                                <label for="password">{"Password"}</label>
                                <Field<Login> input_type="password" form=&self.form name="password" oninput=self.link.callback(|_: InputData| Msg::Update) />
                                <div class=vec!["invalid-feedback", self.form.field_error("password")]>
                                    {&self.form.field_message("password")}
                                </div>
                            </div>
                            <div class="form-group">
                                <Button
                                    loading=self.form.is_processing()
                                    onsignal=self.link.callback(|_| Msg::Submit)
                                    title="Login"
                                />
                            </div>
                            <div class="form-link">
                                <RouterLink to=AppRoute::Register>{"Create new Account"}</RouterLink>
                                <RouterLink to=AppRoute::ResetPassword>{"Reset Password"}</RouterLink>
                            </div>
                        </form>
                    </div>
                </div>
            </main>
        }
    }
}
