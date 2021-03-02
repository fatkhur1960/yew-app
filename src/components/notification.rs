use std::time::Duration;

use yew::prelude::*;
use yew_services::{Task, TimeoutService};

use crate::utils::notif_agent::{NotifAgent, NotifEvent};

use super::Notify;

fn default_pos() -> String {
    String::from("top-right")
}

#[derive(Properties, PartialEq, Clone, Debug)]
pub struct Props {
    #[prop_or_else(default_pos)]
    pub position: String,
}

#[derive(Debug)]
pub enum Msg {
    RemoveNotif(Notify),
    Listen(NotifEvent),
    Done,
}

pub struct Notification {
    props: Props,
    link: ComponentLink<Self>,
    notifs: Vec<Notify>,
    agent: Box<dyn Bridge<NotifAgent>>,
    job: Option<Box<dyn Task>>,
}

impl Notification {
    fn remove<T>(&self, item: Notify) -> Callback<T> {
        self.link.callback(move |_| Msg::RemoveNotif(item.clone()))
    }
}

impl Component for Notification {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let callback = link.callback(Msg::Listen);
        let agent = NotifAgent::bridge(callback);

        Self {
            props,
            link,
            notifs: Vec::new(),
            agent,
            job: None,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        console_log!("got {:?} msg", &msg);
        match msg {
            Msg::RemoveNotif(notif) => {
                self.notifs = self
                    .notifs
                    .clone()
                    .into_iter()
                    .filter(|n| n.ne(&notif))
                    .collect();
                true
            }
            Msg::Listen(event) => match event {
                NotifEvent::Push(notif) => {
                    self.notifs.push(notif.clone());
                    let handle = TimeoutService::spawn(
                        Duration::from_millis(4700),
                        self.link.callback(|_| Msg::Done),
                    );
                    self.job = Some(Box::new(handle));

                    true
                }
                NotifEvent::Remove(notif) => {
                    self.notifs = self
                        .notifs
                        .clone()
                        .into_iter()
                        .filter(|n| n.ne(&notif))
                        .collect();

                    true
                }
            },
            Msg::Done => {
                self.job = None;
                self.notifs.clear();
                true
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div class=format!("notification-container {}", &self.props.position)>
            {
                for self.notifs.iter().map(|child| {
                    let classes = format!("notification box-shadow animate__animated animate__fadeInRight toast {} {}", &child.n_type, &self.props.position);

                    html!{
                        <div class=classes>
                            <div class="toast-buttons">
                                <button onclick=self.remove(child.clone())>
                                    <i class="fas fa-times"></i>
                                </button>
                            </div>
                            <div class="notification-image">
                                <img src=format!("./images/icons/{}.svg", &child.n_type)/>
                            </div>
                            <div>
                                <p class="notification-title">{&child.title}</p>
                                <p class="notification-message">
                                    {&child.description}
                                </p>
                            </div>
                        </div>
                    }
                })
            }
            </div>
        }
    }
}
