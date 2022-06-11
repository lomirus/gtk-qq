use std::sync::Arc;

use qrcode_png::{Color, QrCode, QrCodeEcc};
use rand::prelude::*;
use ricq::{
    device::Device,
    ext::common::after_login,
    version::{get_version, Protocol},
    Client, LoginDeviceLocked, LoginNeedCaptcha, LoginResponse, LoginUnknownStatus,
};
use rusqlite::params;
use tokio::{
    fs::{self, create_dir_all},
    net::TcpStream,
    task,
};

use crate::app::login::{LoginPageMsg, LOGIN_SENDER};
use crate::db::sql::get_db;

use crate::handler::{AppHandler, ACCOUNT, CLIENT};

pub(crate) async fn login(account: i64, password: String) {
    let sender = LOGIN_SENDER.get().unwrap();

    use LoginPageMsg::LoginFailed;
    // Initialize device and client
    let device = Device::random_with_rng(&mut StdRng::seed_from_u64(account as u64));
    let client = Arc::new(Client::new(
        device,
        get_version(Protocol::MacOS),
        AppHandler,
    ));
    // Connect to server
    let stream = match TcpStream::connect(client.get_address()).await {
        Ok(stream) => stream,
        Err(err) => {
            sender.input(LoginFailed(err.to_string()));
            return;
        }
    };
    let client_cloned = client.clone();
    tokio::spawn(async move { client_cloned.start(stream).await });
    task::yield_now().await;
    let res = match client.password_login(account, &password).await {
        Ok(res) => res,
        Err(err) => {
            sender.input(LoginFailed(err.to_string()));
            return;
        }
    };
    handle_login_response(res, account, password, client).await;
}

async fn handle_login_response(
    res: LoginResponse,
    account: i64,
    password: String,
    client: Arc<Client>,
) {
    let sender = LOGIN_SENDER.get().unwrap();

    use LoginPageMsg::LoginFailed;
    match res {
        LoginResponse::Success(_) => {
            finish_login(account, password, client).await;
        }
        LoginResponse::NeedCaptcha(LoginNeedCaptcha { verify_url, .. }) => {
            // Get the captcha url qrcode image path
            let mut path = dirs::home_dir().unwrap();
            path.push(".gtk-qq");
            if let Err(err) = create_dir_all(path.clone()).await {
                sender.input(LoginFailed(err.to_string()));
                return;
            }
            path.push("captcha_url.png");

            // Generate qrcode image
            let verify_url = verify_url.unwrap();
            let mut qrcode = QrCode::new(verify_url.clone(), QrCodeEcc::Low).unwrap();
            qrcode.margin(10);
            qrcode.zoom(5);

            // Write the image
            let buf = qrcode.generate(Color::Grayscale(0, 255)).unwrap();
            if let Err(err) = fs::write(path.clone(), buf).await {
                sender.input(LoginFailed(err.to_string()));
                return;
            };
            sender.input(LoginPageMsg::NeedCaptcha(
                verify_url,
                client.clone(),
                account,
                password,
            ));
        }
        LoginResponse::AccountFrozen => {
            sender.input(LoginFailed("Account Frozen".to_string()));
        }
        LoginResponse::DeviceLocked(LoginDeviceLocked {
            sms_phone,
            verify_url,
            ..
        }) => {
            sender.input(LoginFailed(
                "Device Locked. See more in the pop-up window.".to_string(),
            ));

            sender.input(LoginPageMsg::DeviceLock(
                verify_url.unwrap_or_else(|| "<unknown>".into()),
                sms_phone,
            ));
        }
        LoginResponse::TooManySMSRequest => {
            sender.input(LoginFailed("Too Many SMS Request".to_string()));
        }
        LoginResponse::DeviceLockLogin(_) => {
            if let Err(err) = client.device_lock_login().await {
                sender.input(LoginFailed(err.to_string()));
            } else {
                finish_login(account, password, client).await;
            }
        }
        LoginResponse::UnknownStatus(LoginUnknownStatus { message, .. }) => {
            sender.input(LoginFailed(message));
        }
    }
}

pub(crate) async fn submit_ticket(client: Arc<Client>, ticket: String, account: i64, password: String) {
    let sender = LOGIN_SENDER.get().unwrap();

    match client.submit_ticket(&ticket).await {
        Ok(res) => handle_login_response(res, account, password, client).await,
        Err(err) => {
            sender.input(LoginPageMsg::LoginFailed(err.to_string()));
        }
    }
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
