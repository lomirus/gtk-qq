use std::sync::Arc;

use ricq::{client::Token, LoginResponse};
use tokio::{
    sync::{
        mpsc::{self, error::TrySendError},
        oneshot::{self, error::TryRecvError},
    },
    task::JoinHandle,
};

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

#[derive(Debug)]
pub struct Sender {
    feedback: Option<oneshot::Receiver<()>>,
    tx: mpsc::Sender<(Input, oneshot::Sender<()>)>,
    sender: relm4::Sender<LoginPageMsg>,
}

impl Clone for Sender {
    fn clone(&self) -> Self {
        Self {
            feedback: None,
            tx: self.tx.clone(),
            sender: self.sender.clone(),
        }
    }
}

impl Sender {
    pub fn send(&mut self, input: Input) {
        // check is ready to handle next operate
        if let Some(fb) = &mut self.feedback {
            match fb.try_recv() {
                Ok(_) => {
                    self.feedback.take();
                }
                Err(err) => match err {
                    TryRecvError::Empty => self.sender.send(LoginPageMsg::LoginFailed(
                        "Previous login task not finish yet,please wait".into(),
                    )),
                    TryRecvError::Closed => self
                        .sender
                        .send(LoginPageMsg::LoginFailed("Login Server Closed".into())),
                },
            }
        }
        // ready to handle next operate
        if self.feedback.is_none() {
            let (tx, rx) = oneshot::channel();
            match self.tx.try_send((input, tx)) {
                Ok(_r) => {
                    self.feedback.replace(rx);
                }
                Err(err) => match err {
                    TrySendError::Full(_) => self.sender.send(LoginPageMsg::LoginFailed(
                        "Channel Buff Full,Please wait".into(),
                    )),
                    TrySendError::Closed(_) => self
                        .sender
                        .send(LoginPageMsg::LoginFailed("Login Server Closed".into())),
                },
            }
        }
    }
}

pub struct LoginHandle {
    client: Arc<ricq::Client>,
    rx: mpsc::Receiver<(Input, oneshot::Sender<()>)>,
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
            inner_send: Sender {
                tx,
                sender,
                feedback: None,
            },
        }
    }

    pub fn get_sender(&self) -> Sender {
        self.inner_send.clone()
    }
}

impl LoginHandle {
    pub fn start_handle(mut self) -> JoinHandle<()> {
        let task = async move {
            while let Some((input, sender)) = self.rx.recv().await {
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
                sender.send(()).ok();
            }
        };

        tokio::spawn(task)
    }
}
