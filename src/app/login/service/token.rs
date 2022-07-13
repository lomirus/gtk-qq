use crate::app::login::LoginPageMsg::LoginFailed;
use relm4::Sender;
use ricq::{client::Token, Client};
use rusqlite::params;

use crate::{app::login::LoginPageMsg, db::sql::get_db};

use super::{handle_respond::handle_login_response, init_client};

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
        let db = get_db();
        if let Err(err) = db.execute(
            "REPLACE INTO configs (key, value) VALUES (?1, ?2)",
            params!["account", account],
        ) {
            sender.send(LoginFailed(err.to_string()));
        }
        if let Err(err) = db.execute(
            "REPLACE INTO configs (key, value) VALUES (?1, ?2)",
            params!["token", &token],
        ) {
            sender.send(LoginFailed(err.to_string()));
        }
    }

    pub fn get_account() -> Option<Self> {
        let conn = get_db();
        let mut stmt = conn
            .prepare("SELECT value FROM configs where key='account'")
            .unwrap();
        let mut rows = stmt.query([]).unwrap();

        let next = rows.next().unwrap();

        let account: i64 = next
            .and_then(|row| row.get::<_, String>(0).ok())
            .and_then(|v| v.parse().ok())?;

        let mut stmt = conn
            .prepare("SELECT value FROM configs where key='token'")
            .unwrap();
        let mut rows = stmt.query([]).unwrap();
        let token: String = rows.next().unwrap().and_then(|row| row.get(0).ok())?;

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
        Ok(resp) => handle_login_response(resp, client.clone()).await,
        Err(err) => sender.send(LoginFailed(err.to_string())),
    }
}
