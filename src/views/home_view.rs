use std::{
    borrow::{Borrow, BorrowMut},
    ops::DerefMut,
};

use yew::{
    agent::{Dispatched, Dispatcher},
    prelude::*,
    utils::document,
    Component,
};
use yew::{InputData, MouseEvent};
use yewtil::future::LinkFuture;
use yewtil::NeqAssign;

use crate::{
    components::{
        forms::{Button, Field},
        RouterLink,
    },
    forms::Form,
    models::{AccessToken, Login},
    service::AuthService,
    utils::{
        notif_agent::{self, NotifAgent, NotifEvent},
        RouteEvent, RouterAgent,
    },
    JsonValue, Notify,
};

use super::Props;

pub struct HomeView {
    link: ComponentLink<Self>,
    props: Props,
    form: Form<Login>,
    notif_agent: Dispatcher<NotifAgent>,
}

pub enum Msg {
    Update,
    Submit,
    Authorized(AccessToken),
    Unauthorized(String),
}

impl Component for HomeView {
    type Message = Msg;
    type Properties = Props;

    fn rendered(&mut self, _first_render: bool) {
        document().set_title("Racta - Login")
    }

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            props,
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
            Msg::Authorized(_) => {
                self.form.processing(false);

                true
            }
            Msg::Unauthorized(msg) => {
                self.form.processing(false);
                self.notif_agent.send(NotifEvent::Push(Notify::error(msg)));

                true
            }
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props.neq_assign(props)
    }

    fn view(&self) -> Html {
        html! {
            <main role="main" class="login animate__animated animate__fadeIn">
                <div class="login-container">
                    <div class="app-logo">
                        <img src="./images/racta.png"/>
                    </div>
                    <div class="login-form">
                        <h1 class="login-title">{"Login Your Account"}</h1>
                        <form>
                            <div class="form-group">
                                <label for="email">{"Email"}</label>
                                <Field<Login> input_type="text" form=&self.form field_name="email" oninput=self.link.callback(|_: InputData| Msg::Update) />
                                <div class="invalid-feedback">
                                    {&self.form.field_message("email")}
                                </div>
                            </div>
                            <div class="form-group">
                                <label for="password">{"Password"}</label>
                                <Field<Login> input_type="password" form=&self.form field_name="password" oninput=self.link.callback(|_: InputData| Msg::Update) />
                                <div class="invalid-feedback">
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
                                <RouterLink to="/register" exact=true>{"Create new Account"}</RouterLink>
                                <RouterLink to="/reset-password" exact=true>{"Reset Password"}</RouterLink>
                            </div>
                        </form>
                    </div>
                </div>
            </main>
        }
    }
}
