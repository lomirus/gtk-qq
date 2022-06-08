use std::sync::Arc;

use qrcode_png::{Color, QrCode, QrCodeEcc};
use relm4::{
    adw::{self, Window},
    gtk::{self, Picture},
    ComponentParts, ComponentSender, SimpleComponent, WidgetPlus,
};

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
use tokio::{
    fs::{self, create_dir_all},
    net::TcpStream,
    task,
};

use crate::handler::{AppHandler, ACCOUNT, CLIENT};
use crate::{
    actions::{AboutAction, ShortcutsAction},
    db::sql::get_db,
};
use crate::{app::AppMessage, global::WINDOW};

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
    NeedCaptcha(String, Arc<Client>, i64, String),
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
                    Avatar {
                        set_text: Some("ADW"),
                        set_size: 72,
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
                let window_cloned = window.clone();
                let mut path = dirs::home_dir().unwrap();
                path.push(".gtk-qq");
                path.push("captcha_url.png");
                relm4::view! {
                    ticket_entry = Entry {
                        set_placeholder_text: Some("Paste the ticket here..."),
                        set_margin_end: 8
                    }
                }
                let ticket_buffer = ticket_entry.buffer();
                let verify_url = verify_url.replace('&', "&amp;");
                relm4::view! {
                    vbox = Box {
                        set_orientation: Orientation::Vertical,
                        HeaderBar {
                            set_title_widget = Some(&Label) {
                                set_label: "Captcha Verify Introduction"
                            },
                        },
                        Box {
                            set_valign: Align::Center,
                            set_halign: Align::Center,
                            set_vexpand: true,
                            set_spacing: 24,
                            Box {
                                set_margin_all: 16,
                                set_orientation: Orientation::Vertical,
                                set_halign: Align::Start,
                                set_spacing: 8,
                                Label {
                                    set_xalign: 0.0,
                                    set_markup: r#"1. Install the tool on your android phone: <a href="https://github.com/mzdluo123/TxCaptchaHelper">https://github.com/mzdluo123/TxCaptchaHelper</a>."#,
                                },
                                Label {
                                    set_xalign: 0.0,
                                    set_text: "2. Scan the qrcode and get the ticket."
                                },
                                Box {
                                    Label {
                                        set_text: "3. "
                                    },
                                    append: &ticket_entry,
                                    Button {
                                        set_label: "Submit Ticket",
                                        connect_clicked[sender] => move |_| {
                                            let ticket = ticket_buffer.text();
                                            task::spawn(submit_ticket(client.clone(), ticket, sender.clone(), account, password.clone()));
                                            window_cloned.close();
                                        },
                                    }
                                },
                                Label {
                                    set_xalign: 0.0,
                                    set_markup: &format!(r#"Help: If you do not have an Android phone to install the tool, open the <a href="{}">verify link</a> in the"#, verify_url),

                                },
                                Label {
                                    set_xalign: 0.0,
                                    set_text: "browser manually, open the devtools and switch to the network panel. After you passed the",
                                },
                                Label {
                                    set_xalign: 0.0,
                                    set_text: "verification, you will find a request whose response contains the `ticket`. Then just paste it",
                                },
                                Label {
                                    set_xalign: 0.0,
                                    set_text: "above. The result would be same. It just maybe more complex if you don't know devtools well.",
                                },
                            },
                            Picture {
                                set_filename: Some(&path),
                                set_width_request: 240,
                                set_can_shrink: true
                            },
                        }

                    }
                }
                window.set_content(Some(&vbox));
                window.present();
            }
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
