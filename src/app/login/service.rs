use resource_loader::SyncLoadResource;
use std::{io, sync::Arc};

use qrcode_png::{Color, QrCode};

use ricq::{ext::common::after_login, Client, LoginUnknownStatus};
use rusqlite::params;
use tokio::{net::TcpStream, task};

use crate::app::login::service::handle_respond::handle_login_response;
use crate::app::login::{LoginPageMsg, LOGIN_SENDER};
use crate::db::sql::get_db;

use crate::handler::{AppHandler, ACCOUNT, CLIENT};

pub(super) mod handle_respond;
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



pub(crate) async fn finish_login(account: i64, password: String, client: Arc<Client>) {
    let sender = LOGIN_SENDER.get().unwrap();

    use LoginPageMsg::{LoginFailed, LoginSuccessful};
    if CLIENT.set(client.clone()).is_err() {
        panic!("falied to store client");
    };
    if ACCOUNT.set(account).is_err() {
        panic!("falied to store account");
    };
    // Store user account and password in local database
    let db = get_db();
    if let Err(err) = db.execute(
        "REPLACE INTO configs (key, value) VALUES (?1, ?2)",
        params!["account", account],
    ) {
        sender.input(LoginFailed(err.to_string()));
    }
    if let Err(err) = db.execute(
        "REPLACE INTO configs (key, value) VALUES (?1, ?2)",
        params!["password", password],
    ) {
        sender.input(LoginFailed(err.to_string()));
    }
    // Execute Ricq `after_login()`
    after_login(&client).await;
    sender.input(LoginSuccessful);
}

pub(crate) fn get_login_info() -> (String, String) {
    let conn = get_db();
    let mut stmt = conn
        .prepare("SELECT value FROM configs where key='account'")
        .unwrap();
    let mut rows = stmt.query([]).unwrap();
    let account = match rows.next().unwrap() {
        Some(row) => row.get(0).unwrap(),
        None => String::new(),
    };

    let mut stmt = conn
        .prepare("SELECT value FROM configs where key='password'")
        .unwrap();
    let mut rows = stmt.query([]).unwrap();
    let password = match rows.next().unwrap() {
        Some(row) => row.get(0).unwrap(),
        None => String::new(),
    };

    (account, password)
}
