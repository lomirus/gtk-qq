use relm4::{adw, gtk, Sender};

use adw::{prelude::*, ActionRow, Avatar, PreferencesGroup};
use gtk::{Align, Box, CheckButton, Entry, EntryBuffer, Orientation, PasswordEntry};

use super::{
    payloads::{Input, Payload},
    Output,
};

#[derive(Debug)]
pub struct PwdLoginWidget {
    pub(super) avatar: Avatar,
    pub(super) account: Entry,
    pub(super) pwd: PasswordEntry,
}

impl PwdLoginWidget {
    pub(super) fn new(
        root: &Box,
        payload: &Payload,
        sender: &Sender<Input>,
        output: &Sender<Output>,
    ) -> Self {
        relm4::view! {
            #[name = "input_area"]
            Box {
                set_orientation: Orientation::Vertical,
                set_halign: Align::Center,
                set_valign: Align::Center,
                set_vexpand: true,
                set_spacing: 12,
                PreferencesGroup {
                    add = &ActionRow {
                        set_title: "Account  ",
                        set_focusable: false,
                        add_suffix: account = &Entry {
                            set_valign: Align::Center,
                            set_placeholder_text: Some("QQ account"),
                            connect_changed[sender] => move |entry|{
                                sender.send(Input::Account(entry.buffer().text()))
                            },
                        }
                    },
                    add = &ActionRow {
                        set_title: "Password",
                        set_focusable: false,
                        add_suffix: pwd = &PasswordEntry {
                            set_valign: Align::Center,
                            set_show_peek_icon: true,
                            set_activates_default: true,
                            set_placeholder_text: Some("QQ password"),
                            connect_changed[sender] => move |entry|{
                                sender.send(Input::Password(entry.text().to_string()))
                            },
                            connect_activate[sender] => move |_|{
                                sender.send(Input::Login)
                            },
                        }
                    }
                },
                #[name = "edit_box"]
                Box {
                    set_orientation: Orientation::Horizontal,
                    set_valign: Align::Center,
                    set_halign: Align::Center,
                    set_spacing: 8,
                    append = &CheckButton {
                        set_active: payload.remember_pwd,
                        set_label: Some("Remember Password"),
                        connect_toggled[output] => move |this|{
                            output.send(Output::RememberPwd(this.is_active()));
                        },
                    },
                    append = &CheckButton {
                        set_label: Some("Auto Login"),
                        set_sensitive: false,
                        set_active: payload.auto_login,
                        connect_toggled[output] => move |this|{
                            output.send(Output::AutoLogin(this.is_active()));
                        },
                    }
                }
            }
        }

        relm4::view! {
            #[name = "avatar"]
            Avatar {
                set_size: 96,
            }
        }

        if let Some(ref paintable) = payload.avatar {
            avatar.set_custom_image(Some(paintable));
        }

        if let Some(uin) = payload.account {
            let buf = EntryBuffer::new(Some(&uin.to_string()));
            account.set_buffer(&buf);
        }

        if payload.token.is_some() {
            pwd.set_text("0123456789");
        }

        root.append(&avatar);
        root.append(&input_area);

        Self {
            avatar,
            account,
            pwd,
        }
    }
}
