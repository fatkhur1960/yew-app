use crate::{components::RouterLink, service::ApiError};
use crate::{
    service::github_service::{GithubService, Repsitory},
    service::Result,
};
use yew::{prelude::*, utils::document, Callback};
use yewtil::future::LinkFuture;

use super::Props;

#[use_middleware(AuthMiddleware)]
pub struct BlogView {
    link: ComponentLink<Self>,
    props: Props,
    gs: GithubService,
    repos: Vec<Repsitory>,
}

pub enum Msg {
    ReposLoaded(Vec<Repsitory>),
    ReposLoadError(ApiError),
}

impl Component for BlogView {
    type Message = Msg;
    type Properties = Props;

    fn rendered(&mut self, _first_render: bool) {
        document().set_title("Projects");
    }

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let gs = GithubService::new();
        let gsm = gs.clone();
        let callback = link.callback(Msg::ReposLoaded);

        link.send_future(async move {
            match gsm.get_repos(1, 9).await {
                Ok(res) => Msg::ReposLoaded(res),
                Err(e) => Msg::ReposLoadError(e),
            }
        });

        Self {
            link,
            props,
            gs,
            repos: Vec::new(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::ReposLoaded(repos) => self.repos = repos,
            Msg::ReposLoadError(_) => (),
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <main role="main" class="container animate__animated animate__fadeIn">
              <div class="d-flex align-items-center p-3 my-3 text-white-50 bg-purple rounded box-shadow">
                  <img class="mr-3" src="https://getbootstrap.com/docs/4.0/assets/brand/bootstrap-outline.svg" alt="" width="48" height="48"/>
                  <div class="lh-100">
                      <h6 class="mb-0 text-white lh-100">{"Github Repository"}</h6>
                      <small>{"fatkhur1960"}</small>
                  </div>
              </div>
              <div class="my-3 p-3 bg-white rounded box-shadow">
                  <h6 class="border-bottom border-gray pb-2 mb-0">{"Recent updates"}</h6>
                  {
                      for self.repos.iter().map(|repo| html!{
                          <div class="media text-muted pt-3">
                              <p class="media-body pb-3 mb-0 small lh-125 border-bottom border-gray">
                                  <strong class="d-block text-gray-dark">
                                      <RouterLink to=format!("/project/{}", &repo.name)>{repo.name.clone()}</RouterLink>
                                  </strong>
                                  {repo.description.clone().unwrap_or("".to_string())}
                                  <div class="language">{repo.language.clone().unwrap_or("".to_string())}</div>
                              </p>
                          </div>
                      })
                  }
              </div>
            </main>
        }
    }
}
