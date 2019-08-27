
use failure::Error;
use serde::{Serialize, Deserialize};

use yew::{html, Component, ComponentLink, Html, Renderable, ShouldRender};
use yew::services::ConsoleService;
use yew::services::fetch::{FetchService, FetchTask, Request, Response};
use yew::format::{Json, Nothing};
use yew::services::Task;


pub struct AccountState {
    account_value: String,
    owned_tokens: Vec<Token>,
    owned_identities: Vec<Identity>,
    text: String,
    console: ConsoleService,
    fetcher: FetchService,
    ft: Option<FetchTask>,
    ft2: Option<FetchTask>,
    link: ComponentLink<AccountState>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Token {
    id: u32,
    identityId: u32,
    tokenHash: String,
    cost: String,
    data: String,
    dataType: String,
    expired: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TokenData {
    data: TokenDataOwnedTokens,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TokenDataOwnedTokens {
    ownedTokens: Vec<Token>,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct Identity {
    id: u32,
    ownerId: u32,
    identityHash: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct IdentityData {
    data: IdentityDataOwnedIdentities,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct IdentityDataOwnedIdentities {
    ownedIdentities: Vec<Identity>,
}


pub enum Msg {
    AccountInput(String),
    AccountInputFinished,
    IdentitiesFetchReady(Result<IdentityData, Error>),
    TokensFetchReady(Result<TokenData, Error>),
    Ignore,
}

impl Component for AccountState {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        AccountState {
            account_value: String::new(),
            owned_tokens: vec![],
            owned_identities: vec![],
            text: "ahahahah".to_string(),
            console: ConsoleService::new(),
            fetcher: FetchService::new(),
            ft: None,
            ft2: None,
            link,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::AccountInput(input_str) => {
                self.console.log(&format!("{}", input_str));

                self.account_value = input_str;

                true
            },
            Msg::AccountInputFinished => {

                self.console.log(&format!("Account input value is: {}", self.account_value));

                // fetch owned tokens and identities from server
                // and set to self.
                let url = format!("http://47.254.169.60:3000/graphql");
                self.console.log(&url);
                let body_str = r#"{"query":"{ownedIdentities (address: \"5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY\") { id ownerId identityHash }}\n\n","variables":null,"operationName":null}"#.to_string();
                let request = Request::post(url.as_str())
                    .body(Ok(body_str)).unwrap();

                let callback = self.link.send_back(
                    move |response: Response<Json<Result<IdentityData, Error>>>| {
                        let (meta, Json(data)) = response.into_parts();
                        println!("META: {:?}, {:?}", meta, data);
                        if meta.status.is_success() {
                            Msg::IdentitiesFetchReady(data)
                        } else {
                            Msg::Ignore // FIXME: Handle this error accordingly.
                        }
                    }
                );

                let task = self.fetcher.fetch(request, callback);
                self.ft = Some(task);


                let url = format!("http://47.254.169.60:3000/graphql");
                self.console.log(&url);
                let body_str = r#"{"query":"{\n  ownedTokens (address: \"5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY\") {\n    id\n    identityId\n    ownerId\n    tokenHash\n    cost\n    data\n    dataType\n    expired\n  }\n}\n\n","variables":null,"operationName":null}"#.to_string();
                let request = Request::post(url.as_str())
                    .body(Ok(body_str)).unwrap();

                let callback = self.link.send_back(
                    move |response: Response<Json<Result<TokenData, Error>>>| {
                        let (meta, Json(data)) = response.into_parts();
                        println!("META: {:?}, {:?}", meta, data);
                        if meta.status.is_success() {
                            Msg::TokensFetchReady(data)
                        } else {
                            Msg::Ignore // FIXME: Handle this error accordingly.
                        }
                    }
                );

                let task = self.fetcher.fetch(request, callback);
                self.ft2 = Some(task);

                true
            },
            Msg::IdentitiesFetchReady(response) => {
                self.owned_identities = response.unwrap().data.ownedIdentities;
                
                true
            },
            Msg::TokensFetchReady(response) => {
                self.owned_tokens = response.unwrap().data.ownedTokens;
                
                true
            },
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

impl Renderable<AccountState> for AccountState {
    fn view(&self) -> Html<Self> {
        html! {          
            <div>
                <div class="account">
                    { "Your Account: " }
                    { self.view_account_input() }
                </div>
                <div class="owned_token_list">
                    <div class="title">{ "Owned Token List" }</div>
                    <div class="content">  
                        { for self.owned_tokens.iter().map(Renderable::view) }
                    </div>
                </div>
                <div class="owned_identity_list">
                    <div class="title">{ "Owned Token List" }</div>
                    <div class="content">  
                        { for self.owned_identities.iter().map(Renderable::view) }
                    </div>
                </div>
                <div>
                    { self.text.clone() }
                </div>
            </div>
        }
    }
}

impl AccountState {
    fn view_account_input(&self) -> Html<Self> {
        //format!("Number: {}", self.number)
        html! {
            <input class="account"
                placeholder="Please input your account here"
                value=""
                oninput=|e| Msg::AccountInput(e.value)
                onblur=|e| Msg::AccountInputFinished
                 />
        }
    }
}

impl Renderable<AccountState> for Token {
    fn view(&self) -> Html<AccountState> {
        html! {
            <div class="item"> 
                <span class="caption">{ self.tokenHash.clone() }</span>
                <span class="action">{ "Verify" }</span> 
            </div>
        }
    }
}

impl Renderable<AccountState> for Identity {
    fn view(&self) -> Html<AccountState> {
        html! {
            <div class="item"> 
                <span class="caption">{ self.identityHash.clone() }</span>  
                <span class="action">{ "Generate" }</span> 
            </div>
        }
    }
}