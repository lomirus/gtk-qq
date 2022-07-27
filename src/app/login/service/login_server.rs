use std::{sync::Arc, time::Duration};

use resource_loader::{GetPath, QrCodeLoginCode};
use ricq::{client::Token, Client, LoginResponse};
use tokio::{
    sync::{
        mpsc::{self, error::TrySendError},
        oneshot::{self, error::TryRecvError},
    },
    task::JoinHandle,
    time::interval,
};

use crate::app::login::LoginPageMsg;

use super::{handle_respond::handle_login_response, init_client};

pub enum Login {
    Pwd(i64, String),
    Token(Box<Token>),
}

pub enum Switch {
    Password,
    QrCode,
}

enum LocalState {
    Pwd,
    QrCode(oneshot::Sender<()>, JoinHandle<()>),
}

pub enum Input {
    // switch how to login
    Switch(Switch),
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
            let mut state = LocalState::Pwd;
            let mut timer = interval(Duration::from_millis(2000));

            while let Some((input, sender)) = self.rx.recv().await {
                match (input, &state) {
                    (Input::Login(_), LocalState::QrCode(..)) => {
                        self.sender.send(LoginPageMsg::LoginFailed(
                            "Under `QrCodeLogin` state can not using password login".to_string(),
                        ));
                    }
                    // only work when using password login
                    (Input::Login(login), LocalState::Pwd) => {
                        timer.tick().await;
                        match login {
                            Login::Pwd(account, pwd) => {
                                super::pwd_login::login(account, pwd, &self.sender, &self.client)
                                    .await;
                            }
                            Login::Token(token) => {
                                super::token::token_login(*token, &self.sender, &self.client).await;
                            }
                        }
                    }
                    (Input::LoginRespond(resp), _) => {
                        handle_login_response(&resp, &self.client, &self.sender).await;
                    }
                    (Input::Switch(s), _) => {
                        timer.tick().await;
                        let new_state = match (state, s) {
                            (LocalState::Pwd, Switch::QrCode) => {
                                let (tx, rx) = oneshot::channel();
                                let jh = tokio::spawn(qr_login(
                                    self.client.clone(),
                                    self.sender.clone(),
                                    rx,
                                ));

                                println!("switch to QR");
                                LocalState::QrCode(tx, jh)
                            }
                            (LocalState::QrCode(tx, jh), Switch::Password) => {
                                // send stop signal
                                tx.send(()).ok();
                                // waiting for stop
                                jh.await.ok();
                                println!("switch to PWD");
                                LocalState::Pwd
                            }
                            (state, _) => {
                                // switch to same mod, nothing happen
                                state
                            }
                        };
                        state = new_state;
                    }
                    (Input::Stop, _) => break,
                }
                sender.send(()).ok();
            }
        };

        tokio::spawn(task)
    }
}

async fn qr_login(
    client: Arc<Client>,
    sender: relm4::Sender<LoginPageMsg>,
    mut stopper: oneshot::Receiver<()>,
) {
    use LoginPageMsg::*;
    let temp_path = QrCodeLoginCode::get_path();
    let mut timer = interval(Duration::from_millis(400));
    let mut qrcode_state = match client.fetch_qrcode().await {
        Ok(qrcode) => qrcode,
        Err(err) => {
            sender.send(LoginFailed(err.to_string()));
            return;
        }
    };

    let mut qrcode_sign = Option::None;
    while let Err(TryRecvError::Empty) = stopper.try_recv() {
        match qrcode_state {
            ricq::QRCodeState::ImageFetch(ref qrcode) => {
                let img = &qrcode.image_data;
                tokio::fs::write(temp_path, &img)
                    .await
                    .expect("failure to write qrcode file");
                qrcode_sign.replace(qrcode.sig.clone());
                sender.send(UpdateQrCode);
                println!("QrCode fetch ,save in {:?}", temp_path);
            }
            ricq::QRCodeState::WaitingForScan => {
                println!("waiting for scan qrcode");
            }
            ricq::QRCodeState::WaitingForConfirm => {
                println!("waiting for qrcode confirm");
            }
            ricq::QRCodeState::Timeout => {
                println!("QrCode Timeout fetch again");
                match client.fetch_qrcode().await {
                    Ok(qr_state) => {
                        qrcode_state = qr_state;
                        continue;
                    }
                    Err(err) => {
                        sender.send(LoginFailed(err.to_string()));
                        return;
                    }
                }
            }
            ricq::QRCodeState::Confirmed(ref qrcode_confirm) => {
                println!("QrCode confirmed, ready for login");
                let login_respond = client
                    .qrcode_login(
                        &qrcode_confirm.tmp_pwd,
                        &qrcode_confirm.tmp_no_pic_sig,
                        &qrcode_confirm.tgt_qr,
                    )
                    .await;
                match login_respond {
                    Ok(ok_respond) => sender.send(LoginRespond(ok_respond.into())),
                    Err(err) => sender.send(LoginFailed(err.to_string())),
                }
                return;
            }
            ricq::QRCodeState::Canceled => {
                println!("Canceled QrCode");
                sender.send(LoginFailed("QrCode login has been canceled".into()));
                return;
            }
        }

        timer.tick().await;
        let qrcode_sig = qrcode_sign
            .as_ref()
            .map(|byte| -> &[u8] { byte })
            .unwrap_or(&[]);
        qrcode_state = match client.query_qrcode_result(qrcode_sig).await {
            Ok(state) => state,
            Err(err) => {
                sender.send(LoginFailed(err.to_string()));
                return;
            }
        }
    }
}
