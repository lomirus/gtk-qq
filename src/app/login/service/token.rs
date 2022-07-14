use crate::{
    app::login::LoginPageMsg::{LoginFailed, LoginRespond},
    db::sql::{load_sql_config, save_sql_config},
};
use relm4::Sender;
use ricq::{client::Token, Client};

use crate::app::login::LoginPageMsg;

use super::init_client;

pub struct LocalAccount {
    pub account: i64,
    pub token: Token,
}

impl LocalAccount {
    fn token_to_base64(token: &Token) -> String {
        let vec = bincode::serialize(&token).expect("serde token error");
        base64::encode(vec)
    }

    fn base64_to_token(base64: &str) -> Token {
        let vec = base64::decode(base64).expect("Bad Base64 Encode");
        bincode::deserialize(&vec).expect("Bad Bincode format")
    }

    pub async fn new(client: &Client) -> Self {
        let uin = client.uin().await;
        let token = client.gen_token().await;

        Self {
            account: uin,
            token,
        }
    }

    pub fn save_account(&self, sender: &Sender<LoginPageMsg>) {
        let account = self.account.to_string();
        let token = Self::token_to_base64(&self.token);

        let saving = || {
            save_sql_config(&"account", &account)?;
            save_sql_config(&"token", &token)
        };
        if let Err(err) = saving() {
            sender.send(LoginFailed(err.to_string()));
        }
    }

    pub fn get_account() -> Option<Self> {
        let account: i64 = load_sql_config(&"account")
            .ok()
            .flatten()
            .and_then(|v| v.parse().ok())?;

        let token = load_sql_config(&"token").ok().flatten()?;
        let token = Self::base64_to_token(&token);

        Some(Self { account, token })
    }
}

pub async fn token_login(token: Token, sender: Sender<LoginPageMsg>) {
    let client = match init_client().await {
        Ok(client) => client,
        Err(err) => {
            sender.send(LoginFailed(err.to_string()));
            return;
        }
    };

    match client.token_login(token).await {
        Ok(resp) => sender.send(LoginRespond(resp.into(), client)),
        Err(err) => sender.send(LoginFailed(err.to_string())),
    }
}
