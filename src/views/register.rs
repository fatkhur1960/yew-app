use yew::InputData;
use yew::{
    agent::{Dispatched, Dispatcher},
    prelude::*,
    utils::document,
    Component,
};

use crate::{
    forms::{Button, Field, Form},
    models::{AccessToken, Register},
    service::AuthService,
    utils::{
        notif_agent::{NotifAgent, NotifEvent},
        token,
    },
    AppRoute, Notify, RouterLink,
};

pub struct RegisterView {
    link: ComponentLink<Self>,
    form: Form<Register>,
    notif_agent: Dispatcher<NotifAgent>,
}

pub enum Msg {
    Update,
    Submit,
    Authorized(AccessToken),
    Unauthorized(String),
}

impl Component for RegisterView {
    type Message = Msg;
    type Properties = ();

    fn rendered(&mut self, first_render: bool) {
        document().set_title("Racta - Register");

        if first_render && token::is_authenticated() {
            redirect!("/projects");
        }
    }

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            form: Form::new(Register {
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
                    let auth = AuthService::new();
                    let body = self.form.model();
                    self.form.processing(true);

                    // self.link.send_future(async move {
                    //     match auth.authorize(&body).await {
                    //         Ok(res) => {
                    //             if res.code == 0 {
                    //                 Msg::Authorized(res.result.unwrap())
                    //             } else {
                    //                 Msg::Unauthorized(res.description)
                    //             }
                    //         }
                    //         Err(e) => Msg::Unauthorized(e.msg),
                    //     }
                    // });
                }

                true
            }
            Msg::Authorized(at) => {
                self.form.processing(false);
                token::set_token(Some(at.token));
                redirect!("/projects");

                true
            }
            Msg::Unauthorized(msg) => {
                self.form.processing(false);
                self.notif_agent.send(NotifEvent::Push(Notify::error(msg)));

                true
            }
        }
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let oninput = &self.link.callback(|_: InputData| Msg::Update);

        html! {
            <main role="main" class="login animate__animated animate__fadeIn">
                <div class="login-container">
                    <div class="app-logo">
                        <img src="/images/racta.png"/>
                    </div>
                    <div class="login-form">
                        <h1 class="login-title">{"Register Your Account"}</h1>
                        <form>
                            <div class="form-group">
                                <label for="full_name">{"Full Name"}</label>
                                <Field<Register> maxlength=30 input_type="text" form=&self.form name="full_name" oninput=oninput />
                                <div class=vec!["invalid-feedback", self.form.field_error("full_name")]>
                                    {&self.form.field_message("full_name")}
                                </div>
                            </div>
                            <div class="form-group">
                                <label for="nickname">{"Nick Name"}</label>
                                <Field<Register> maxlength=15 input_type="text" form=&self.form name="nickname" oninput=oninput />
                                <div class=vec!["invalid-feedback", self.form.field_error("nickname")]>
                                    {&self.form.field_message("nickname")}
                                </div>
                            </div>
                            <div class="form-group">
                                <label for="email">{"Email"}</label>
                                <Field<Register> input_type="text" form=&self.form name="email" oninput=oninput />
                                <div class=vec!["invalid-feedback", self.form.field_error("email")]>
                                    {&self.form.field_message("email")}
                                </div>
                            </div>
                            <div class="form-group">
                                <label for="phone_num">{"Phone Number"}</label>
                                <Field<Register> input_type="text" form=&self.form name="phone_num" oninput=oninput />
                            </div>
                            <div class="form-group">
                                <label for="password">{"Password"}</label>
                                <Field<Register> input_type="password" form=&self.form name="password" oninput=oninput />
                                <div class=vec!["invalid-feedback", self.form.field_error("password")]>
                                    {&self.form.field_message("password")}
                                </div>
                            </div>
                            <div class="form-group">
                                <label for="confirm_password">{"Confirm Password"}</label>
                                <Field<Register> input_type="password" form=&self.form name="confirm_password" oninput=oninput />
                                <div class=vec!["invalid-feedback", self.form.field_error("confirm_password")]>
                                    {&self.form.field_message("confirm_password")}
                                </div>
                            </div>
                            <div class="form-group">
                                <Button
                                    loading=self.form.is_processing()
                                    onsignal=self.link.callback(|_| Msg::Submit)
                                    title="Register"
                                />
                            </div>
                            <div class="form-register-link">
                                <div>
                                    {"Already have an account? "} <RouterLink to=AppRoute::Login>{"Login"}</RouterLink>
                                </div>
                            </div>
                        </form>
                    </div>
                </div>
            </main>
        }
    }
}
