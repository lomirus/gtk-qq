mod captcha;
mod device_lock;
mod service;

use crate::db::sql::{load_sql_config, save_sql_config};
use crate::gtk::Button;
use std::cell::RefCell;
use std::sync::Arc;

use once_cell::sync::OnceCell;
use relm4::gtk::gdk::Paintable;
use relm4::{
    adw, gtk, Component, ComponentController, ComponentParts, ComponentSender, SimpleComponent,
};

use adw::prelude::*;
use adw::{HeaderBar, Toast, ToastOverlay, Window};

use gtk::gdk_pixbuf::Pixbuf;
use gtk::{Box, Label, MenuButton, Orientation, Picture};

use ricq::client::Token;
use ricq::Client;
use tokio::task;
use widgets::pwd_login::{self, Input, PasswordLogin, PasswordLoginModel, Payload};

use crate::actions::{AboutAction, ShortcutsAction};
use crate::app::AppMessage;
use crate::db::fs::{download_user_avatar_file, get_user_avatar_path};
use crate::global::WINDOW;

use self::service::pwd_login::login;
use self::service::token::{token_login, LocalAccount};

type SmsPhone = Option<String>;
type VerifyUrl = String;

pub static LOGIN_SENDER: OnceCell<ComponentSender<LoginPageModel>> = OnceCell::new();

#[derive(Debug)]
pub struct LoginPageModel {
    pwd_login: PasswordLogin,
    enable_btn: bool,
    remember_pwd: bool,
    auto_login: bool,
    toast: RefCell<Option<String>>,
}

pub enum LoginPageMsg {
    StartLogin,
    PwdLogin(i64, String),
    TokenLogin(Token),
    EnableLogin(bool),
    LoginSuccessful,
    LoginFailed(String),
    NeedCaptcha(String, Arc<Client>),
    DeviceLock(VerifyUrl, SmsPhone),
    ConfirmVerification,
    LinkCopied,
    RememberPwd(bool),
    AutoLogin(bool),
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
        if LOGIN_SENDER.set(sender.clone()).is_err() {
            panic!("failed to initialize login sender");
        }
        let remember_pwd = load_sql_config(&"remember_pwd")
            .ok()
            .flatten()
            .and_then(|v| v.parse().ok())
            .unwrap_or(false);

        let auto_login = load_sql_config(&"auto_login")
            .ok()
            .flatten()
            .and_then(|v| v.parse().ok())
            .unwrap_or(false);

        let account = if !remember_pwd {
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
                remember_pwd,
                auto_login,
            })
            .forward(sender.input_sender(), |out| match out {
                pwd_login::Output::Login { account, pwd } => LoginPageMsg::PwdLogin(account, pwd),
                pwd_login::Output::EnableLogin(enable) => LoginPageMsg::EnableLogin(enable),
                pwd_login::Output::TokenLogin(token) => LoginPageMsg::TokenLogin(token),
                pwd_login::Output::RememberPwd(b) => LoginPageMsg::RememberPwd(b),
                pwd_login::Output::AutoLogin(b) => LoginPageMsg::AutoLogin(b),
            });

        let widgets = view_output!();
        let model = LoginPageModel {
            pwd_login,
            toast: RefCell::new(None),
            enable_btn: false,
            remember_pwd,
            auto_login,
        };

        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: LoginPageMsg, sender: &ComponentSender<Self>) {
        use LoginPageMsg::*;
        match msg {
            RememberPwd(b) => {
                self.remember_pwd = b;
            }
            AutoLogin(b) => {
                self.auto_login = b;
            }
            TokenLogin(token) => {
                let sender = sender.input_sender().clone();
                task::spawn(token_login(token, sender));
            }
            EnableLogin(enable) => {
                self.enable_btn = enable;
            }
            StartLogin => {
                self.pwd_login.emit(Input::Login);
            }
            PwdLogin(uin, pwd) => {
                task::spawn(login(uin, pwd));
            }
            LoginSuccessful => {
                self.save_login_setting();
                sender.output(AppMessage::LoginSuccessful);
            }
            LoginFailed(msg) => {
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
            LinkCopied => {
                self.toast.borrow_mut().replace("Link Copied".into());
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
            ConfirmVerification => self.pwd_login.emit(Input::Login),
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
                }
            },
            #[name = "toast_overlay"]
            ToastOverlay {
                set_child : Some(pwd_login.widget()),
            }
        }
    }

    fn pre_view(&self, widgets: &mut Self::Widgets, sender: &ComponentSender<Self>) {
        if let Some(ref content) = self.toast.borrow_mut().take() {
            widgets.toast_overlay.add_toast(&Toast::new(&content));
        }
        widgets.login_btn.set_sensitive(self.enable_btn);
    }

    fn shutdown(&mut self, _: &mut Self::Widgets, _: relm4::Sender<Self::Output>) {
        self.save_login_setting()
    }
}

impl LoginPageModel {
    fn save_login_setting(&self){
        save_sql_config(&"remember_pwd", &self.remember_pwd.to_string()).expect("Save cfg Error");
        save_sql_config(&"auto_login", &self.auto_login.to_string()).expect("Save cfg Error");

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
