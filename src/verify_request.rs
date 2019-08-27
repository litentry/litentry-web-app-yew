
use failure::Error;
use serde::{Serialize, Deserialize};

use yew::{html, Component, ComponentLink, Html, Renderable, ShouldRender};
use yew::services::ConsoleService;
use yew::services::fetch::{FetchService, FetchTask, Request, Response};
use yew::format::{Json, Nothing};
use yew::services::Task;


pub struct VerifyRequest {

}


pub enum Msg {
    Ignore,
}

impl Component for VerifyRequest {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        VerifyRequest {
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

impl Renderable<VerifyRequest> for VerifyRequest {
    fn view(&self) -> Html<Self> {
        html! {
            <div>
                <div class="account">
                    { "Your Account: " }
                    { "0x5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY" }
                </div>
                <div class="token_info">
                    <div class="item id">{ "Token Id: " } { "0x874908275880af321" }</div>
                    <div class="item identity">{ "Issued Identity: " } { "0x4343b341f24a9999999" }</div>
                </div>
                <div class="content qrcode">
                    <div class="title">{ "Webcam Scan QR Code" }</div>
                </div>
                <div>
                    { "(Success or Not!)" }
                </div>
            </div>
        }
    }
}

