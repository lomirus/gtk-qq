// TODO: temp tag waiting for other impl
#![allow(dead_code)]

use crate::app::login::LoginPageMsg::LoginFailed;
use relm4::Sender;
use ricq::{client::Token, Client};
use rusqlite::params;

use crate::{app::login::LoginPageMsg, db::sql::get_db};

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
        let account: i64 = rows.next().unwrap().and_then(|row| row.get(0).ok())?;

        let mut stmt = conn
            .prepare("SELECT value FROM configs where key='token'")
            .unwrap();
        let mut rows = stmt.query([]).unwrap();
        let token: String = rows.next().unwrap().and_then(|row| row.get(0).ok())?;

        let token = Self::base64_to_token(&token);

        Some(Self { account, token })
    }
}