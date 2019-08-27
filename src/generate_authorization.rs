
use failure::Error;
use serde::{Serialize, Deserialize};

use yew::{html, Component, ComponentLink, Html, Renderable, ShouldRender};
use yew::services::ConsoleService;
use yew::services::fetch::{FetchService, FetchTask, Request, Response};
use yew::format::{Json, Nothing};
use yew::services::Task;


pub struct GenerateAuthorization {

}


pub enum Msg {
    Ignore,
}

impl Component for GenerateAuthorization {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        GenerateAuthorization {
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Ignore => {
                false
            }
        }
    }
    
    /*
    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        // Apparently change MUST be implemented in this case, even though no props were changed
        true
    }
    */
}

impl Renderable<GenerateAuthorization> for GenerateAuthorization {
    fn view(&self) -> Html<Self> {
        html! {
            <div>
                <div class="account">
                    { "Your Account: " }
                    { "0x5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY" }
                </div>
                <div class="destination">
                    { "Destination Account: " } 
                    <span>{ "" }</span>
                </div>
                <div class="issued_identity">
                    { "Issued Identity: " } 
                    <span>{ "" }</span>
                </div>
                <div class="data">
                    { "Data: " } 
                    <span>{ "" }</span>
                </div>
                <div>
                    <button onclick=|_| Msg::Ignore>{ "Generate!" }</button>
                </div>
            </div>
        }
    }
}

