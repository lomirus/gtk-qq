use std::sync::Arc;

use ricq::{client::Token, Client, LoginResponse};
use tokio::{sync::mpsc, task::JoinHandle};

use crate::app::login::LoginPageMsg;

use super::{handle_respond::handle_login_response, init_client};

pub enum Login {
    Pwd(i64, String),
    Token(Token),
    QrCode,
}

pub enum Input {
    // login
    Login(Login),
    // login proc
    LoginRespond(Box<LoginResponse>),
    Stop,
}

#[derive(Debug, Clone)]
pub struct Sender {
    tx: mpsc::Sender<Input>,
    sender: relm4::Sender<LoginPageMsg>,
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
    pub fn start_handle(mut self) -> JoinHandle<Arc<Client>> {
        let task = async move {
            while let Some(input) = self.rx.recv().await {
                match input {
                    Input::Login(login) => match login {
                        Login::Pwd(account, pwd) => {
                            super::pwd_login::login(account, pwd, &self.sender).await;
                        }
                        Login::Token(token) => {
                            super::token::token_login(token, &self.sender).await;
                        }
                        Login::QrCode => {
                            todo!()
                        }
                    },
                    Input::LoginRespond(resp) => {
                        handle_login_response(&resp, Arc::clone(&self.client), &self.sender).await;
                    }
                    Input::Stop => break,
                }
            }

            self.client
        };

        tokio::spawn(task)
    }
}
