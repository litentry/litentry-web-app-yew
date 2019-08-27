#![recursion_limit = "256"]

use yew::{html, Component, ComponentLink, Html, Renderable, ShouldRender};

mod account_state;
use account_state::AccountState as PanelAccountState;
mod verify_request;
use verify_request::VerifyRequest as PanelVerifyRequest;
mod generate_authorization;
use generate_authorization::GenerateAuthorization as PanelGenerateAuthorization;

pub struct Model {
    body: BodyPanel,
}

pub enum BodyPanel {
    PanelAccountState,
    PanelVerifyRequest,
    PanelGenerateAuthorization,
    PathNotFound(String),
}

pub enum Msg {
    NavigateTo(BodyPanel),
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Model {
            body: BodyPanel::PanelAccountState,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::NavigateTo(body) => {
                self.body = body;

                true
            }
        }
    }
}

impl Renderable<Model> for Model {
    fn view(&self) -> Html<Self> {
        html! {
            <div>
                <span>
                    { "Litentry & Logo" }
                </span>
                <nav class="menu">
                    <button onclick=|_| Msg::NavigateTo(BodyPanel::PanelAccountState)>{ "Account State" }</button>
                    <button onclick=|_| Msg::NavigateTo(BodyPanel::PanelVerifyRequest)>{ "Verify Request" }</button>
                    <button onclick=|_| Msg::NavigateTo(BodyPanel::PanelGenerateAuthorization)>{ "Generate Authorization" }</button>
                </nav>
                <div>
                    {self.body.view()}
                </div>
            </div>
        }
    }
}

impl Renderable<Model> for BodyPanel {
    fn view(&self) -> Html<Model> {
        match *self {
            BodyPanel::PanelAccountState => html! {
                <>
                    <PanelAccountState />
                </>
            },
            BodyPanel::PanelVerifyRequest => html! {
                <>
                    <PanelVerifyRequest />
                </>
            },
            BodyPanel::PanelGenerateAuthorization => html! {
                <>
                    <PanelGenerateAuthorization />
                </>
            },
            BodyPanel::PathNotFound(ref path) => html! {
                <>
                    {format!("Invalid path: '{}'", path)}
                </>
            },
        }
    }
}
