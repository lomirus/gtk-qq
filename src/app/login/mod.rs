mod captcha;
mod device_lock;
mod service;

use crate::app::login::service::login_server::{Login, Switch};
use crate::db::sql::{load_sql_config, save_sql_config};
use crate::gtk::Button;
use std::boxed;
use std::cell::RefCell;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use relm4::gtk::gdk::Paintable;
use relm4::gtk::{Align, Stack};
use relm4::{
    adw, gtk, Component, ComponentController, ComponentParts, ComponentSender, SimpleComponent,
};

use adw::prelude::*;
use adw::{HeaderBar, Toast, ToastOverlay, Window};

use gtk::gdk_pixbuf::Pixbuf;
use gtk::{Box, Label, MenuButton, Orientation, Picture};

use resource_loader::GetPath;
use ricq::client::Token;
use ricq::{Client, LoginResponse};
use tokio::task;
use widgets::pwd_login::{self, Input, PasswordLogin, PasswordLoginModel, Payload};
use widgets::qrcode_login::{self, QrCodeLogin, QrCodeLoginModel};

use crate::actions::{AboutAction, ShortcutsAction};
use crate::app::AppMessage;
use crate::db::fs::{download_user_avatar_file, get_user_avatar_path};
use crate::global::WINDOW;

use self::service::login_server::{self, LoginHandle, Sender};
use self::service::token::LocalAccount;

type SmsPhone = Option<String>;
type VerifyUrl = String;

pub(in crate::app::login) static REMEMBER_PWD: AtomicBool = AtomicBool::new(false);
pub(in crate::app::login) static AUTO_LOGIN: AtomicBool = AtomicBool::new(false);

#[derive(Debug, Default)]
pub enum LoginState {
    #[default]
    Password,
    QrCode,
}

#[derive(Debug)]
pub struct LoginPageModel {
    btn_enabled: bool,
    is_logging: bool,
    pwd_login: PasswordLogin,
    qr_code_login: QrCodeLogin,
    toast: RefCell<Option<String>>,
    sender: Option<Sender>,
    login_state: LoginState,
}

pub enum LoginPageMsg {
    ClientInit(LoginHandle),

    LoginSwitch(LoginState),

    StartLogin,
    PwdLogin(i64, String),
    TokenLogin(Token),
    LoginRespond(boxed::Box<LoginResponse>),
    LoginSuccessful(Arc<Client>),

    LoginFailed(String),
    NeedCaptcha(String, Arc<Client>),
    DeviceLock(VerifyUrl, SmsPhone),
    ConfirmVerification,

    EnableLogin(bool),
    RememberPwd(bool),
    AutoLogin(bool),
    UpdateQrCode,

    LinkCopied,
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
        // start client
        let t_sender = sender.input_sender().clone();
        tokio::spawn(async move {
            t_sender.send(LoginPageMsg::ClientInit(
                LoginHandle::new(t_sender.clone()).await,
            ))
        });

        // load config
        REMEMBER_PWD.store(
            load_sql_config("remember_pwd")
                .ok()
                .flatten()
                .and_then(|v| v.parse().ok())
                .unwrap_or(false),
            Ordering::Relaxed,
        );

        AUTO_LOGIN.store(
            load_sql_config("auto_login")
                .ok()
                .flatten()
                .and_then(|v| v.parse().ok())
                .unwrap_or(false),
            Ordering::Relaxed,
        );

        // load safe account
        let account = if !REMEMBER_PWD.load(Ordering::Relaxed) {
            None
        } else {
            LocalAccount::get_account()
        };
        let account_ref = Into::<Option<&LocalAccount>>::into(&account);
        let avatar = load_avatar(account_ref.map(|a| a.account), true);

        let pwd_login = PasswordLoginModel::builder()
            .launch(Payload {
                account: account_ref.map(|a| a.account),
                avatar,
                token: account.map(|a| a.token),
                remember_pwd: REMEMBER_PWD.load(Ordering::Relaxed),
                auto_login: AUTO_LOGIN.load(Ordering::Relaxed),
            })
            .forward(sender.input_sender(), |out| match out {
                pwd_login::Output::Login { account, pwd } => LoginPageMsg::PwdLogin(account, pwd),
                pwd_login::Output::EnableLogin(enable) => LoginPageMsg::EnableLogin(enable),
                pwd_login::Output::TokenLogin(token) => LoginPageMsg::TokenLogin(token),
                pwd_login::Output::RememberPwd(b) => LoginPageMsg::RememberPwd(b),
                pwd_login::Output::AutoLogin(b) => LoginPageMsg::AutoLogin(b),
            });

        let qr_code_login = QrCodeLoginModel::builder()
            .launch(widgets::qrcode_login::PayLoad {
                temp_img_path: resource_loader::QrCodeLoginCode::get_path(),
            })
            //TODO: forward Qr login Event
            .detach();

        let widgets = view_output!();

        let model = LoginPageModel {
            btn_enabled: false,
            is_logging: false,
            pwd_login,
            qr_code_login,
            login_state: Default::default(),
            toast: RefCell::new(None),
            sender: None,
        };

        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: LoginPageMsg, sender: &ComponentSender<Self>) {
        use LoginPageMsg::*;
        match msg {
            UpdateQrCode => {
                self.qr_code_login.emit(qrcode_login::Input::UpdateQrCode);
            }
            ClientInit(client) => {
                self.sender.replace(client.get_sender());
                client.start_handle();
            }
            LoginSwitch(target) => {
                match (&target, &mut self.sender) {
                    (LoginState::Password, Some(sender)) => {
                        sender.send(login_server::Input::Switch(Switch::Password))
                    }
                    (LoginState::QrCode, Some(sender)) => {
                        sender.send(login_server::Input::Switch(Switch::QrCode))
                    }
                    (_, _) => sender.input(LoginFailed("Client Not Init. Please Wait".into())),
                };
                self.login_state = target;
            }
            LoginRespond(boxed_login_resp) => {
                if let Some(sender) = &mut self.sender {
                    sender.send(login_server::Input::LoginRespond(boxed_login_resp))
                } else {
                    sender.input(LoginFailed("Client Not Init. Please Wait".into()));
                }
            }
            RememberPwd(b) => {
                REMEMBER_PWD.store(b, Ordering::Relaxed);
            }
            AutoLogin(b) => {
                AUTO_LOGIN.store(b, Ordering::Relaxed);
            }
            TokenLogin(token) => {
                if let Some(sender) = &mut self.sender {
                    sender.send(login_server::Input::Login(Login::Token(token.into())))
                } else {
                    sender.input(LoginFailed("Client Not Init. Please Wait".into()));
                }
            }
            EnableLogin(enabled) => {
                self.btn_enabled = enabled && self.sender.is_some() && !self.is_logging;
            }
            StartLogin => {
                self.btn_enabled = false;
                self.is_logging = true;
                self.pwd_login.emit(Input::Login);
            }
            PwdLogin(uin, pwd) => {
                if let Some(sender) = &mut self.sender {
                    sender.send(login_server::Input::Login(Login::Pwd(uin, pwd)))
                } else {
                    sender.input(LoginFailed("Client Not Init. Please Wait".into()));
                }
            }
            LoginSuccessful(_) => {
                self.save_login_setting();
                sender.output(AppMessage::LoginSuccessful);
            }
            LoginFailed(msg) => {
                self.btn_enabled = true;
                self.is_logging = false;
                *(self.toast.borrow_mut()) = Some(msg);
            }
            NeedCaptcha(verify_url, client) => {
                sender.input(LoginPageMsg::LoginFailed(
                    "Need Captcha. See more in the pop-up window.".to_string(),
                ));
                let window = Window::builder()
                    .transient_for(&WINDOW.get().unwrap().window)
                    .default_width(640)
                    .build();

                window.connect_destroy(|_| println!("closed window"));

                let verify_url = verify_url.replace('&', "&amp;");

                let captcha = captcha::CaptchaModel::builder()
                    .launch(captcha::PayLoad {
                        client: Arc::clone(&client),
                        verify_url,
                        window: window.clone(),
                    })
                    .forward(sender.input_sender(), |output| output);

                window.set_content(Some(captcha.widget()));
                window.present();
            }

            DeviceLock(verify_url, sms) => {
                let window = Window::builder()
                    .transient_for(&WINDOW.get().unwrap().window)
                    .default_width(640)
                    .build();

                let device_lock = device_lock::DeviceLock::builder()
                    .launch(device_lock::Payload {
                        window: window.clone(),
                        unlock_url: verify_url,
                        sms_phone: sms,
                    })
                    .forward(sender.input_sender(), |output| output);

                window.set_content(Some(device_lock.widget()));
                window.present()
            }
            // TODO: proc follow operate
            ConfirmVerification => sender.input(LoginPageMsg::StartLogin),
            LinkCopied => {
                self.toast.borrow_mut().replace("Link Copied".into());
            }
        }
    }

    fn shutdown(&mut self, _: &mut Self::Widgets, _: relm4::Sender<Self::Output>) {
        self.save_login_setting()
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
                pack_end : login_btn = &Button{
                    set_icon_name : "go-next",
                    set_sensitive : false,
                    connect_clicked[sender] => move |_|{
                        sender.input(LoginPageMsg::StartLogin)
                    }
                },
                pack_end = &MenuButton {
                    set_icon_name: "menu-symbolic",
                    set_menu_model: Some(&main_menu),
                },
                pack_end : switch = &Button{
                    set_label : "QrCode",
                    connect_clicked[sender] => move |this|{
                        if this.label().unwrap() == "QrCode"{
                            this.set_label("Password");
                            sender.input(LoginPageMsg::LoginSwitch(LoginState::QrCode));
                        }else{
                            this.set_label("QrCode");
                            sender.input(LoginPageMsg::LoginSwitch(LoginState::Password));
                        }
                    }
                },
            },
            #[name = "toast_overlay"]
            ToastOverlay {
                set_child  =  Some(&gtk::Box){
                    set_orientation:gtk::Orientation::Vertical,
                    set_halign:Align::Center,
                    set_valign:Align::Center,
                    append : stack= &Stack{
                        set_halign:Align::Center,
                        set_valign:Align::Center,
                        add_child : pwd_login_box = &gtk::Box {
                            append : pwd_login.widget(),
                        },
                        add_child: qr_code_login_box = &gtk::Box{
                            append: qr_code_login.widget(),
                        }
                    }
                }
            }
        }
    }

    fn pre_view(&self, widgets: &mut Self::Widgets, sender: &ComponentSender<Self>) {
        if let Some(ref content) = self.toast.borrow_mut().take() {
            widgets.toast_overlay.add_toast(&Toast::new(content));
        }
        widgets.login_btn.set_sensitive(self.btn_enabled);

        match self.login_state {
            LoginState::Password => stack.set_visible_child(pwd_login_box),
            LoginState::QrCode => stack.set_visible_child(qr_code_login_box),
        }
    }
}

impl LoginPageModel {
    fn save_login_setting(&self) {
        save_sql_config(
            "remember_pwd",
            REMEMBER_PWD.load(Ordering::Relaxed).to_string(),
        )
        .expect("Save cfg Error");
        save_sql_config("auto_login", AUTO_LOGIN.load(Ordering::Relaxed).to_string())
            .expect("Save cfg Error");
    }
}

fn load_avatar(account: Option<i64>, auto_download: bool) -> Option<Paintable> {
    account
        .map(|uin| (uin, get_user_avatar_path(uin)))
        .and_then(|(uin, path)| {
            if path.exists() {
                Some(path)
            } else {
                if auto_download {
                    task::spawn(download_user_avatar_file(uin));
                }
                None
            }
        })
        .and_then(|path| Pixbuf::from_file_at_size(path, 96, 96).ok())
        .map(|pix| Picture::for_pixbuf(&pix))
        .and_then(|pic| pic.paintable())
}
