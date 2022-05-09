use std::collections::VecDeque;

use relm4::{adw, gtk, send, ComponentUpdate, Model, Sender, Widgets};

use adw::prelude::*;
use adw::{ActionRow, Avatar, HeaderBar, PreferencesGroup, Toast, ToastOverlay};
use gtk::{Align, Box, Button, Entry, Label, Orientation};

use crate::{AppModel, Message};

#[derive(Default, Debug)]
pub struct LoginPageModel {
    account: String,
    password: String,
    toast_stack: VecDeque<String>,
}

impl Model for LoginPageModel {
    type Msg = LoginPageMsg;
    type Widgets = LoginPageWidgets;
    type Components = ();
}

pub enum LoginPageMsg {
    LoginStart,
    LoginSuccessful,
    AccountChange(String),
    PasswordChange(String),
    PushToast(String),
    ShiftToast
}

impl ComponentUpdate<AppModel> for LoginPageModel {
    fn init_model(_parent_model: &AppModel) -> Self {
        LoginPageModel::default()
    }

    fn update(
        &mut self,
        msg: LoginPageMsg,
        _components: &(),
        sender: Sender<LoginPageMsg>,
        parent_sender: Sender<Message>,
    ) {
        use LoginPageMsg::*;
        match msg {
            LoginStart => {
                println!("{:?}", self);
                if self.account == "" {
                    send!(sender, PushToast("Account cannot be empty".to_string()));
                    return;
                }
                if self.password == "" {
                    send!(sender, PushToast("Password cannot be empty".to_string()));
                    return;
                }
                send!(sender, LoginPageMsg::LoginSuccessful)
            }
            LoginSuccessful => send!(parent_sender, Message::LoginSuccessful),
            AccountChange(new_account) => self.account = new_account,
            PasswordChange(new_password) => self.password = new_password,
            PushToast(message) => self.toast_stack.push_back(message),
            ShiftToast => {
                self.toast_stack.pop_front().expect("failed to pop from toast stack");
            }
        }
    }
}

#[relm4::widget(pub)]
impl Widgets<LoginPageModel, AppModel> for LoginPageWidgets {
    view! {
        &Box {
            set_hexpand: true,
            set_vexpand: true,
            set_orientation: Orientation::Vertical,
            append = &HeaderBar {
                set_title_widget = Some(&Label) {
                    set_label: "Login"
                },
                pack_end = &Button {
                    set_icon_name: "go-next",
                    connect_clicked(sender) => move |_| {
                        send!(sender, LoginPageMsg::LoginStart);
                    },
                },
                pack_end: &Button::from_icon_name("dialog-information"),
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
                            add_suffix = &Entry {
                                set_valign: Align::Center,
                                set_placeholder_text: Some("请输入您的QQ号码"),
                                connect_changed(sender) => move |e| {
                                    sender.send(LoginPageMsg::AccountChange(e.buffer().text())).unwrap();
                                }
                            },
                        },
                        add = &ActionRow {
                            set_title: "Password",
                            add_suffix = &Entry {
                                set_valign: Align::Center,
                                set_placeholder_text: Some("请输入您的QQ密码"),
                                connect_changed(sender) => move |e| {
                                    sender.send(LoginPageMsg::PasswordChange(e.buffer().text())).unwrap();
                                }
                            },
                        },
                    },
                },
            },
        }
    }

    fn pre_view() {
        if !model.toast_stack.is_empty() {
            let toast_message = model.toast_stack[0].as_str();
            send!(sender, LoginPageMsg::ShiftToast);
            self.toast_overlay.add_toast(&Toast::new(toast_message));
        }
    }
}
