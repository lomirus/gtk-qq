use std::sync::Arc;

use qrcode_png::{Color, QrCode, QrCodeEcc};
use relm4::{
    adw, gtk, Component, ComponentController, ComponentParts, ComponentSender, SimpleComponent,
};

use adw::prelude::*;
use adw::{ActionRow, Avatar, HeaderBar, PreferencesGroup, Toast, ToastOverlay, Window};
use gtk::gdk::Paintable;
use gtk::gdk_pixbuf::Pixbuf;
use gtk::{Align, Box, Button, Entry, EntryBuffer, Label, MenuButton, Orientation, Picture};

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
use widgets::{captcha, device_lock};

use crate::{
    actions::{AboutAction, ShortcutsAction},
    db::sql::get_db,
};
use crate::{app::AppMessage, global::WINDOW};
use crate::{
    db::fs::{download_user_avatar_file, get_user_avatar_path},
    handler::{AppHandler, ACCOUNT, CLIENT},
};

type SmsPhone = Option<String>;
type VerifyUrl = String;
type UserId = i64;
type Password = String;

#[derive(Debug)]
pub struct LoginPageModel {
    account: String,
    password: String,
    is_login_button_enabled: bool,
    toast: Option<String>,
}

pub enum LoginPageMsg {
    LoginStart,
    LoginSuccessful,
    LoginFailed(String),
    AccountChange(String),
    PasswordChange(String),
    NeedCaptcha(String, Arc<Client>, UserId, Password),
    SubmitTicket(String, Arc<Client>, UserId, Password),
    DeviceLock(VerifyUrl, SmsPhone),
    ConfirmVerified,
    CopyLink,
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
    tokio::spawn(async move { client_cloned.start(stream).await });
    task::yield_now().await;
    let res = match client.password_login(account, &password).await {
        Ok(res) => res,
        Err(err) => {
            sender.input(LoginFailed(err.to_string()));
            return;
        }
    };
    handle_login_response(res, account, password, client, sender).await;
}

async fn handle_login_response(
    res: LoginResponse,
    account: i64,
    password: String,
    client: Arc<Client>,
    sender: ComponentSender<LoginPageModel>,
) {
    use LoginPageMsg::LoginFailed;
    match res {
        LoginResponse::Success(_) => {
            finish_login(account, password, client, sender).await;
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
            message,
            ..
        }) => {
            sender.input(LoginFailed(
                "Device Locked. See more in the pop-up window.".to_string(),
            ));

            println!("------[TODO: Add GUI for this]");
            println!("message: {:?}", message);
            println!("sms_phone: {:?}", sms_phone);
            println!("verify_url: {:?}", verify_url);

            sender.input(LoginPageMsg::DeviceLock(
                verify_url.unwrap_or("<unknown>".into()),
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
                finish_login(account, password, client, sender).await;
            }
        }
        LoginResponse::UnknownStatus(LoginUnknownStatus { message, .. }) => {
            sender.input(LoginFailed(message));
        }
    }
}

async fn submit_ticket(
    client: Arc<Client>,
    ticket: String,
    sender: ComponentSender<LoginPageModel>,
    account: i64,
    password: String,
) {
    match client.submit_ticket(&ticket).await {
        Ok(res) => handle_login_response(res, account, password, client, sender).await,
        Err(err) => {
            sender.input(LoginPageMsg::LoginFailed(err.to_string()));
        }
    }
}

async fn finish_login(
    account: i64,
    password: String,
    client: Arc<Client>,
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
    type Input = LoginPageMsg;
    type Output = AppMessage;
    type InitParams = ();
    type Widgets = LoginPageWidgets;

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

        if let Ok(account) = account.parse::<i64>() {
            let path = get_user_avatar_path(account);
            if path.exists() {
                if let Ok(pixbuf) = Pixbuf::from_file_at_size(path, 96, 96) {
                    let image = Picture::for_pixbuf(&pixbuf);
                    if let Some(paintable) = image.paintable() {
                        widgets.avatar.set_custom_image(Some(&paintable));
                    }
                }
            } else {
                task::spawn(download_user_avatar_file(account));
            }
        }

        let model = LoginPageModel {
            account,
            password,
            is_login_button_enabled: true,
            toast: None,
        };

        ComponentParts { model, widgets }
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
            NeedCaptcha(verify_url, client, account, password) => {
                sender.input(LoginPageMsg::LoginFailed(
                    "Need Captcha. See more in the pop-up window.".to_string(),
                ));
                let window = Window::builder()
                    .transient_for(&WINDOW.get().unwrap().window)
                    .default_width(640)
                    .build();
                println!("{:?}", WINDOW.get().unwrap().window);
                window.connect_destroy(|_| println!("closed window"));
                let mut path = dirs::home_dir().unwrap();
                path.push(".gtk-qq");
                path.push("captcha_url.png");

                let verify_url = verify_url.replace('&', "&amp;");

                let captcha = widgets::captcha::CaptchaModel::builder()
                    .launch(
                        captcha::PayLoad::builder()
                            .verify_url(verify_url)
                            .window(window.clone())
                            .build(),
                    )
                    .forward(sender.input_sender(), move |out| match out {
                        captcha::Output::Submit { ticket } => {
                            SubmitTicket(ticket, Arc::clone(&client), account, password.clone())
                        }
                        captcha::Output::CopyLink => CopyLink,
                    });

                window.set_content(Some(captcha.widget()));
                window.present();
            }
            SubmitTicket(t, c, account, pwd) => {
                task::spawn(submit_ticket(c, t, sender.clone(), account, pwd));
            }
            CopyLink => {
                self.toast.replace("Link Copied".into());
            }
            DeviceLock(verify_url, sms) => {
                let window = Window::builder()
                    .transient_for(&WINDOW.get().unwrap().window)
                    .default_width(640)
                    .build();

                let device_lock = device_lock::DeviceLock::builder()
                    .launch(
                        device_lock::Payload::builder()
                            .window(window.clone())
                            .unlock_url(verify_url)
                            .sms_phone(sms)
                            .build(),
                    )
                    .forward(sender.input_sender(), move |out| match out {
                        device_lock::Output::ConfirmVerify => ConfirmVerified,
                        device_lock::Output::CopyLink => CopyLink,
                    });

                window.set_content(Some(device_lock.widget()));
                window.present()
            }
            //TODO: proc follow operate
            ConfirmVerified => sender.input(LoginStart),
        }
    }

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
            #[name = "headerbar"]
            HeaderBar {
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
            #[name = "toast_overlay"]
            ToastOverlay {
                set_child = Some(&Box) {
                    set_halign: Align::Center,
                    set_valign: Align::Center,
                    set_vexpand: true,
                    set_spacing: 32,
                    #[name = "avatar"]
                    Avatar {
                        set_size: 96,
                    },
                    PreferencesGroup {
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

    fn pre_view(&self, widgets: &mut Self::Widgets, sender: &ComponentSender<Self>) {
        if let Some(content) = &self.toast {
            widgets.toast_overlay.add_toast(&Toast::new(content));
        }
        widgets
            .go_next_button
            .set_sensitive(self.is_login_button_enabled);

        // TODO: IF ELSE HELL!!! Someone helps improve here please.
        if let Ok(account) = self.account.parse::<i64>() {
            let path = get_user_avatar_path(account);
            if path.exists() {
                if let Ok(pixbuf) = Pixbuf::from_file_at_size(path, 96, 96) {
                    let image = Picture::for_pixbuf(&pixbuf);
                    if let Some(paintable) = image.paintable() {
                        widgets.avatar.set_custom_image(Some(&paintable));
                    } else {
                        widgets.avatar.set_custom_image(Option::<&Paintable>::None);
                    }
                } else {
                    widgets.avatar.set_custom_image(Option::<&Paintable>::None);
                }
            } else {
                widgets.avatar.set_custom_image(Option::<&Paintable>::None);
            }
        } else {
            widgets.avatar.set_custom_image(Option::<&Paintable>::None);
        }
    }
}
