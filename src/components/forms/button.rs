use yew::events::MouseEvent;
use yew::prelude::*;
use yewtil::NeqAssign;

pub struct Button {
    link: ComponentLink<Self>,
    props: Props,
}

pub enum Msg {
    Clicked(MouseEvent),
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub title: String,
    pub onsignal: Callback<MouseEvent>,
    #[prop_or_default]
    pub loading: bool,
}

impl Component for Button {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Button {
            link,
            props,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Clicked(e) => {
                e.prevent_default();
                self.props.onsignal.emit(e);
            }
        }
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props.neq_assign(props)
    }

    fn view(&self) -> Html {
        html! {
            <button class="btn btn-primary" disabled=self.props.loading
                onclick=self.link.callback(Msg::Clicked)>
                {
                    if self.props.loading {
                        html! {
                            <span 
                                class="spinner-border" 
                                style="width: 1.5rem; height: 1.5rem;" 
                                role="status" aria-hidden=true>
                            </span>
                        }
                    } else {
                        html!{ &self.props.title }
                    }
                }
            </button>
        }
    }
}
