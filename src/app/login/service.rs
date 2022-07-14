use relm4::Sender;
use resource_loader::SyncLoadResource;
use std::{io, sync::Arc};

use qrcode_png::{Color, QrCode};

use ricq::{ext::common::after_login, Client, LoginUnknownStatus};
use tokio::{net::TcpStream, task};

use crate::app::login::service::token::LocalAccount;
use crate::app::login::LoginPageMsg;

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
    let stream = TcpStream::connect(client.get_address()).await?;
    let client_cloned = client.clone();
    tokio::spawn(async move { client_cloned.start(stream).await });
    task::yield_now().await;

    Ok(client)
}

pub(crate) async fn finish_login(client: Arc<Client>, sender: &Sender<LoginPageMsg>) {
    let local = LocalAccount::new(&client).await;

    use LoginPageMsg::LoginSuccessful;
    if CLIENT.set(client.clone()).is_err() {
        panic!("falied to store client");
    };
    if ACCOUNT.set(local.account).is_err() {
        panic!("falied to store account");
    };

    local.save_account(&sender);

    after_login(&client).await;
    sender.send(LoginSuccessful);
}
