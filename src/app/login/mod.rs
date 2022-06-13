mod captcha;
mod device_lock;
mod service;

use std::sync::Arc;

use once_cell::sync::OnceCell;
use relm4::{
    adw, gtk, Component, ComponentController, ComponentParts, ComponentSender, SimpleComponent,
};

use adw::prelude::*;
use adw::{ActionRow, Avatar, HeaderBar, PreferencesGroup, Toast, ToastOverlay, Window};
use gtk::gdk::Paintable;
use gtk::gdk_pixbuf::Pixbuf;
use gtk::{Align, Box, Button, Entry, EntryBuffer, Label, MenuButton, Orientation, Picture};

use ricq::Client;
use tokio::task;

use crate::actions::{AboutAction, ShortcutsAction};
use crate::app::AppMessage;
use crate::db::fs::{download_user_avatar_file, get_user_avatar_path};
use crate::global::WINDOW;
use crate::utils::avatar::loader::{AvatarLoader, User};

use self::service::{get_login_info, login};

type SmsPhone = Option<String>;
type VerifyUrl = String;
type UserId = i64;
type Password = String;

pub static LOGIN_SENDER: OnceCell<ComponentSender<LoginPageModel>> = OnceCell::new();

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
    DeviceLock(VerifyUrl, SmsPhone),
    ConfirmVerification,
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
        if LOGIN_SENDER.set(sender.clone()).is_err() {
            panic!("failed to initialize login sender");
        }

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
                task::spawn(login(account, password));
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

                window.connect_destroy(|_| println!("closed window"));

                let verify_url = verify_url.replace('&', "&amp;");

                let captcha = captcha::CaptchaModel::builder()
                    .launch(captcha::PayLoad {
                        client: Arc::clone(&client),
                        verify_url,
                        window: window.clone(),
                        account,
                        password,
                    })
                    .forward(sender.input_sender(), |output| output);

                window.set_content(Some(captcha.widget()));
                window.present();
            }
            LinkCopied => {
                self.toast.replace("Link Copied".into());
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
            ConfirmVerification => sender.input(LoginStart),
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

        let paint = self
            .account
            .parse::<i64>()
            .ok()
            .and_then(|id| User::get_avatar_as_pixbuf(id, 96, 96).ok())
            .map(|pix_buf| Picture::for_pixbuf(&pix_buf))
            .and_then(|pic| pic.paintable());

        widgets
            .avatar
            .set_custom_image(Into::<Option<&'_ Paintable>>::into(&paint));
    }
}
