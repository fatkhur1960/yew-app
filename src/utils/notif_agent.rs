use std::collections::HashSet;

use yew::worker::*;

use crate::Notify;

/// Internal Message used for the RouteAgent.
#[derive(Debug)]
pub enum Msg {}

#[derive(Debug, Clone)]
pub enum NotifEvent {
    /// Add notif.
    Push(Notify),
    /// Remove notif
    Remove(Notify),
}

#[derive(Clone)]
pub struct NotifAgent {
    link: AgentLink<Self>,
    subscribers: HashSet<HandlerId>,
}

impl Agent for NotifAgent {
    type Reach = Context<Self>;
    type Message = Msg;
    type Input = NotifEvent;
    type Output = NotifEvent;

    fn create(link: AgentLink<Self>) -> Self {
        Self {
            link,
            subscribers: HashSet::new(),
        }
    }

    fn update(&mut self, _msg: Self::Message) {}

    fn handle_input(&mut self, msg: Self::Input, _: HandlerId) {
        for subs in self.subscribers.iter() {
            self.link.respond(*subs, msg.clone());
        }
    }

    fn connected(&mut self, id: HandlerId) {
        self.subscribers.insert(id);
    }

    fn disconnected(&mut self, id: HandlerId) {
        self.subscribers.remove(&id);
    }
}
