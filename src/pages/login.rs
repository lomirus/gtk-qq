use relm4::{adw, gtk, send, ComponentUpdate, Model, Sender, Widgets};

use adw::HeaderBar;
use gtk::{Align, Box, Button, Entry, Grid, Label, Orientation};

use adw::prelude::*;

use crate::{AppModel, Message};

pub struct LoginPageModel;

impl Model for LoginPageModel {
    type Msg = LoginPageMsg;
    type Widgets = LoginPageWidgets;
    type Components = ();
}

pub enum LoginPageMsg {
    LoginSuccessful,
}

impl ComponentUpdate<AppModel> for LoginPageModel {
    fn init_model(_parent_model: &AppModel) -> Self {
        LoginPageModel
    }
    fn update(
        &mut self,
        msg: LoginPageMsg,
        _components: &(),
        _sender: Sender<LoginPageMsg>,
        parent_sender: Sender<Message>,
    ) {
        match msg {
            LoginPageMsg::LoginSuccessful => send!(parent_sender, Message::LoginSuccessful),
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
                        send!(sender, LoginPageMsg::LoginSuccessful);
                    },
                },
                pack_end: &Button::from_icon_name("dialog-information"),
            },
            append = &Box {
                set_halign: Align::Center,
                set_valign: Align::Center,
                set_vexpand: true,
                set_spacing: 32,
                append = &adw::Avatar {
                    set_text: Some("ADW"),
                    set_size: 72,
                },
                append = &Grid {
                    set_row_spacing: 12,
                    set_column_spacing: 12,
                    attach(0, 0, 1, 1) = &Label {
                        set_label: "Account"
                    },
                    attach(1, 0, 1, 1) = &Entry {
                        set_placeholder_text: Some("请输入您的QQ号码")
                    },
                    attach(0, 1, 1, 1) = &Label {
                        set_label: "Password"
                    },
                    attach(1, 1, 1, 1) = &Entry {
                        set_placeholder_text: Some("请输入您的QQ密码")
                    }
                }
            }
        }
    }
}
