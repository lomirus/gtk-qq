use relm4::Sender;
use resource_loader::SyncLoadResource;
use std::{
    io,
    sync::{atomic::Ordering, Arc},
};

use qrcode_png::{Color, QrCode};

use ricq::{
    client::{Connector, DefaultConnector},
    ext::common::after_login,
    Client, LoginUnknownStatus,
};
use tokio::task;

use crate::app::login::{service::token::LocalAccount, LoginPageMsg, REMEMBER_PWD};

use crate::handler::{AppHandler, ACCOUNT, CLIENT};

pub(super) mod handle_respond;
pub mod login_server;
pub(super) mod pwd_login;
pub mod token;

pub(crate) async fn init_client() -> io::Result<Arc<Client>> {
    let client = Arc::new(Client::new(
        resource_loader::Device::load_resource(()).unwrap(),
        resource_loader::Protocol::load_resource(()).unwrap(),
        AppHandler,
    ));

    // Connect to server
    tokio::spawn({
        let client = client.clone();
        // 连接所有服务器，哪个最快用哪个，可以使用 TcpStream::connect 代替
        let stream = DefaultConnector.connect(&client).await.unwrap();
        async move { client.start(stream).await }
    });

    task::yield_now().await;

    Ok(client)
}

pub(crate) async fn finish_login(client: Arc<Client>, sender: &Sender<LoginPageMsg>) {
    let local = LocalAccount::new(&client).await;

    use LoginPageMsg::LoginSuccessful;
    if CLIENT.set(client.clone()).is_err() {
        panic!("failed to store client");
    };
    if ACCOUNT.set(local.account).is_err() {
        panic!("failed to store account");
    };
    if REMEMBER_PWD.load(Ordering::Relaxed) {
        local.save_account(sender);
    }

    after_login(&client).await;
    sender.send(LoginSuccessful(client));
}
