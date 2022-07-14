use std::sync::Arc;

use ricq::{client::Token, LoginResponse};
use tokio::{sync::mpsc, task::JoinHandle};

use crate::app::login::LoginPageMsg;

use super::{handle_respond::handle_login_response, init_client};

pub enum Login {
    Pwd(i64, String),
    Token(Box<Token>),
    #[allow(dead_code)]
    QrCode,
}

pub enum Input {
    // login
    Login(Login),
    // login proc
    LoginRespond(Box<LoginResponse>),
    #[allow(dead_code)]
    Stop,
}

#[derive(Debug, Clone)]
pub struct Sender {
    tx: mpsc::Sender<Input>,
    sender: relm4::Sender<LoginPageMsg>,
}

impl Sender {
    pub fn send(&self, input: Input) {
        let sender = self.sender.clone();
        let tx = self.tx.clone();
        tokio::spawn(async move {
            if tx.send(input).await.is_err() {
                sender.send(LoginPageMsg::LoginFailed(
                    "Login in Handle receive end closed".into(),
                ))
            }
        });
    }
}

pub struct LoginHandle {
    client: Arc<ricq::Client>,
    rx: mpsc::Receiver<Input>,
    sender: relm4::Sender<LoginPageMsg>,
    inner_send: Sender,
}

impl LoginHandle {
    pub async fn new(sender: relm4::Sender<LoginPageMsg>) -> LoginHandle {
        let client = init_client().await.expect("Init Client Error");
        let (tx, rx) = mpsc::channel(8);

        Self {
            client,
            rx,
            sender: sender.clone(),
            inner_send: Sender { tx, sender },
        }
    }

    pub fn get_sender(&self) -> Sender {
        self.inner_send.clone()
    }
}

impl LoginHandle {
    pub fn start_handle(mut self) -> JoinHandle<()> {
        let task = async move {
            while let Some(input) = self.rx.recv().await {
                match input {
                    Input::Login(login) => match login {
                        Login::Pwd(account, pwd) => {
                            super::pwd_login::login(account, pwd, &self.sender, &self.client).await;
                        }
                        Login::Token(token) => {
                            super::token::token_login(*token, &self.sender, &self.client).await;
                        }
                        Login::QrCode => {
                            todo!()
                        }
                    },
                    Input::LoginRespond(resp) => {
                        handle_login_response(&resp, &self.client, &self.sender).await;
                    }
                    Input::Stop => break,
                }
            }
        };

        tokio::spawn(task)
    }
}
