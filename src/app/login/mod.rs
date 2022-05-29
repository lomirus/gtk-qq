use std::collections::VecDeque;
use std::sync::Arc;

use relm4::{adw, gtk, ComponentParts, ComponentSender, JoinHandle, SimpleComponent};

use adw::{prelude::*, ActionRow, Avatar, HeaderBar, PreferencesGroup, Toast, ToastOverlay};
use gtk::{Align, Box, Button, Entry, Label, MenuButton, Orientation};

use rand::prelude::*;
use ricq::{
    device::Device,
    ext::common::after_login,
    version::{get_version, Protocol},
    Client, LoginDeviceLocked, LoginNeedCaptcha, LoginResponse, LoginUnknownStatus,
};
use tokio::{net::TcpStream, task};

use crate::actions::{AboutAction, ShortcutsAction};
use crate::app::main::{MainMsg, MAIN_SENDER};
use crate::app::AppMessage;
use crate::handler::{init_friends_list, AppHandler, ACCOUNT, CLIENT, GROUP_LIST};

#[derive(Debug)]
pub struct LoginPageModel {
    account: String,
    password: String,
    is_login_button_enabled: bool,
    toast_stack: VecDeque<String>,
}

impl Default for LoginPageModel {
    fn default() -> Self {
        LoginPageModel {
            account: String::new(),
            password: String::new(),
            is_login_button_enabled: true,
            toast_stack: VecDeque::<String>::new(),
        }
    }
}

#[derive(Debug)]
pub enum LoginPageMsg {
    LoginStart,
    LoginSuccessful,
    LoginFailed(String),
    AccountChange(String),
    PasswordChange(String),
    PushToast(String),
    ShiftToast,
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
            finish_login(account, client, handle, sender).await;
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
                finish_login(account, client, handle, sender).await;
            }
        }
        LoginResponse::UnknownStatus(LoginUnknownStatus { message, .. }) => {
            sender.input(LoginFailed(message));
        }
    }
}

async fn finish_login(
    account: i64,
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
    after_login(&client).await;
    match client.get_friend_list().await {
        Ok(res) => init_friends_list(res.friends, res.friend_groups),
        Err(err) => {
            sender.input(LoginFailed(err.to_string()));
            return;
        }
    };
    match client.get_group_list().await {
        Ok(res) => GROUP_LIST.set(res).unwrap(),
        Err(err) => {
            sender.input(LoginFailed(err.to_string()));
            return;
        }
    };
    sender.input(LoginSuccessful);
    handle.await.unwrap();
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
                            add_suffix = &Entry {
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
                            add_suffix = &Entry {
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
                        sender.input(PushToast("Account is invalid".to_string()));
                        return;
                    }
                };
                // Get the password
                let password = if self.password.is_empty() {
                    sender.input(PushToast("Password cannot be empty".to_string()));
                    return;
                } else {
                    self.password.to_string()
                };

                self.is_login_button_enabled = false;
                task::spawn(login(account, password, sender.clone()));
            }
            LoginSuccessful => {
                MAIN_SENDER.get().unwrap().input(MainMsg::InitSidebar);
                sender.output(AppMessage::LoginSuccessful);
            }
            LoginFailed(msg) => {
                sender.input(PushToast(msg));
                self.is_login_button_enabled = true;
            }
            AccountChange(new_account) => self.account = new_account,
            PasswordChange(new_password) => self.password = new_password,
            PushToast(message) => self.toast_stack.push_back(message),
            ShiftToast => {
                self.toast_stack
                    .pop_front()
                    .expect("failed to pop from toast stack");
            }
        }
    }

    fn init(
        _init_params: Self::InitParams,
        root: &Self::Root,
        sender: &ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let widgets = view_output!();
        let model = LoginPageModel::default();
        ComponentParts { model, widgets }
    }

    fn pre_view(&self, widgets: &mut Self::Widgets, sender: &ComponentSender<Self>) {
        if !self.toast_stack.is_empty() {
            let toast_message = self.toast_stack[0].as_str();
            sender.input(LoginPageMsg::ShiftToast);
            widgets.toast_overlay.add_toast(&Toast::new(toast_message));
        }
        widgets
            .go_next_button
            .set_sensitive(self.is_login_button_enabled);
    }
}
