use std::sync::Arc;

use relm4::{adw, gtk, ComponentParts, ComponentSender, JoinHandle, SimpleComponent};

use adw::{prelude::*, ActionRow, Avatar, HeaderBar, PreferencesGroup, Toast, ToastOverlay};
use gtk::{Align, Box, Button, Entry, EntryBuffer, Label, MenuButton, Orientation};

use rand::prelude::*;
use ricq::{
    device::Device,
    ext::common::after_login,
    version::{get_version, Protocol},
    Client, LoginDeviceLocked, LoginNeedCaptcha, LoginResponse, LoginUnknownStatus,
};
use rusqlite::params;
use tokio::{net::TcpStream, task};

use crate::app::AppMessage;
use crate::handler::{AppHandler, ACCOUNT, CLIENT};
use crate::{
    actions::{AboutAction, ShortcutsAction},
    db::sql::get_db,
};

#[derive(Debug)]
pub struct LoginPageModel {
    account: String,
    password: String,
    is_login_button_enabled: bool,
    toast: Option<String>,
}

#[derive(Debug)]
pub enum LoginPageMsg {
    LoginStart,
    LoginSuccessful,
    LoginFailed(String),
    AccountChange(String),
    PasswordChange(String),
}

async fn login(account: i64, password: String, sender: ComponentSender<LoginPageModel>) {
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
    let handle = tokio::spawn(async move { client_cloned.start(stream).await });
    task::yield_now().await;
    let res = match client.password_login(account, &password).await {
        Ok(res) => res,
        Err(err) => {
            sender.input(LoginFailed(err.to_string()));
            return;
        }
    };
    // Handle login response
    match res {
        LoginResponse::Success(_) => {
            finish_login(account, password, client, handle, sender).await;
        }
        LoginResponse::NeedCaptcha(LoginNeedCaptcha {
            verify_url,
            image_captcha,
            ..
        }) => {
            sender.input(LoginFailed(
                "Need Captcha. See more in the console.".to_string(),
            ));
            println!("------[TODO: Add GUI for this]");
            println!("verify_url: {:?}", verify_url);
            println!("image_captcha: {:?}", image_captcha);
        }
        LoginResponse::AccountFrozen => {
            sender.input(LoginFailed("Account Frozen".to_string()));
        }
        LoginResponse::DeviceLocked(LoginDeviceLocked {
            sms_phone,
            verify_url,
            message,
            ..
        }) => {
            sender.input(LoginFailed(
                "Device Locked. See more in the console.".to_string(),
            ));
            println!("------[TODO: Add GUI for this]");
            println!("message: {:?}", message);
            println!("sms_phone: {:?}", sms_phone);
            println!("verify_url: {:?}", verify_url);
        }
        LoginResponse::TooManySMSRequest => {
            sender.input(LoginFailed("Too Many SMS Request".to_string()));
        }
        LoginResponse::DeviceLockLogin(_) => {
            if let Err(err) = client.device_lock_login().await {
                sender.input(LoginFailed(err.to_string()));
            } else {
                finish_login(account, password, client, handle, sender).await;
            }
        }
        LoginResponse::UnknownStatus(LoginUnknownStatus { message, .. }) => {
            sender.input(LoginFailed(message));
        }
    }
}

async fn finish_login(
    account: i64,
    password: String,
    client: Arc<Client>,
    handle: JoinHandle<()>,
    sender: ComponentSender<LoginPageModel>,
) {
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
    handle.await.unwrap();
}

fn get_login_info() -> (String, String) {
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

#[relm4::component(pub)]
impl SimpleComponent for LoginPageModel {
    type Widgets = LoginPageWidgets;
    type InitParams = ();
    type Input = LoginPageMsg;
    type Output = AppMessage;

    menu! {
        main_menu: {
            "Keyboard Shortcuts" => ShortcutsAction,
            "About Gtk QQ" => AboutAction
        }
    }

    view! {
        login_page = Box {
            set_hexpand: true,
            set_vexpand: true,
            set_orientation: Orientation::Vertical,
            append: headerbar = &HeaderBar {
                set_title_widget = Some(&Label) {
                    set_label: "Login"
                },
                pack_end: go_next_button = &Button {
                    set_icon_name: "go-next",
                    connect_clicked[sender] => move |_| {
                        sender.input(LoginPageMsg::LoginStart);
                    },
                },
                pack_end = &MenuButton {
                    set_icon_name: "menu-symbolic",
                    set_menu_model: Some(&main_menu),
                }
            },
            append: toast_overlay = &ToastOverlay {
                set_child = Some(&Box) {
                    set_halign: Align::Center,
                    set_valign: Align::Center,
                    set_vexpand: true,
                    set_spacing: 32,
                    append = &Avatar {
                        set_text: Some("ADW"),
                        set_size: 72,
                    },
                    append = &PreferencesGroup {
                        add = &ActionRow {
                            set_title: "Account",
                            set_focusable: false,
                            add_suffix: account_entry = &Entry {
                                set_valign: Align::Center,
                                set_placeholder_text: Some("Please input your QQ account "),
                                connect_changed[sender] => move |e| {
                                    sender.input(LoginPageMsg::AccountChange(e.buffer().text()));
                                }
                            },
                        },
                        add = &ActionRow {
                            set_title: "Password",
                            set_focusable: false,
                            add_suffix: password_entry = &Entry {
                                set_valign: Align::Center,
                                set_placeholder_text: Some("Please input your QQ password"),
                                set_visibility: false,
                                connect_changed[sender] => move |e| {
                                    sender.input(LoginPageMsg::PasswordChange(e.buffer().text()));
                                }
                            },
                        },
                    },
                },
            }
        }
    }

    fn update(&mut self, msg: LoginPageMsg, sender: &ComponentSender<Self>) {
        use LoginPageMsg::*;
        match msg {
            LoginStart => {
                // Get the account
                let account: i64 = match self.account.parse::<i64>() {
                    Ok(account) => account,
                    Err(_) => {
                        self.toast = Some("Account is invalid".to_string());
                        return;
                    }
                };
                // Get the password
                let password = if self.password.is_empty() {
                    self.toast = Some("Password cannot be empty".to_string());
                    return;
                } else {
                    self.password.to_string()
                };

                self.is_login_button_enabled = false;
                task::spawn(login(account, password, sender.clone()));
            }
            LoginSuccessful => {
                sender.output(AppMessage::LoginSuccessful);
            }
            LoginFailed(msg) => {
                self.toast = Some(msg);
                self.is_login_button_enabled = true;
            }
            AccountChange(new_account) => self.account = new_account,
            PasswordChange(new_password) => self.password = new_password,
        }
    }

    fn init(
        _init_params: Self::InitParams,
        root: &Self::Root,
        sender: &ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let widgets = view_output!();

        let (account, password) = get_login_info();
        let account_buffer = EntryBuffer::new(Some(&account));
        let password_buffer = EntryBuffer::new(Some(&password));
        widgets.account_entry.set_buffer(&account_buffer);
        widgets.password_entry.set_buffer(&password_buffer);

        let model = LoginPageModel {
            account,
            password,
            is_login_button_enabled: true,
            toast: None,
        };

        ComponentParts { model, widgets }
    }

    fn pre_view(&self, widgets: &mut Self::Widgets, sender: &ComponentSender<Self>) {
        if let Some(content) = &self.toast {
            widgets.toast_overlay.add_toast(&Toast::new(content));
        }
        widgets
            .go_next_button
            .set_sensitive(self.is_login_button_enabled);
    }
}
