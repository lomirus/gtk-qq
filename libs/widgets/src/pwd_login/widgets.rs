use relm4::{
    adw::{
        traits::{ActionRowExt, PreferencesGroupExt},
        ActionRow, Avatar, PreferencesGroup,
    },
    gtk::{
        self,
        prelude::EntryBufferExtManual,
        traits::{BoxExt, ButtonExt, EditableExt, EntryExt},
        Align, Button, Entry, PasswordEntry,
    },
    Sender,
};

use super::payloads::{Input, Payload};

pub struct PwdLoginWidget {
    pub(super) avatar: Avatar,
    _group: PreferencesGroup,
    _account_row: ActionRow,
    pub(super) account: Entry,
    _pwd_row: ActionRow,
    pub(super) pwd: PasswordEntry,
    pub(super) login_btn: Button,
}

impl PwdLoginWidget {
    pub(super) fn new(root: &gtk::Box, payload: &Payload, sender: &Sender<Input>) -> Self {
        let avatar = Avatar::builder().size(96).build();

        if let Some(ref a) = payload.avatar {
            avatar.set_custom_image(Some(a));
        }

        let _group = PreferencesGroup::new();
        let _account_row = ActionRow::builder()
            .title("Account  ")
            .focusable(false)
            .build();

        let account = Entry::builder()
            .valign(Align::Center)
            .placeholder_text("Please input your QQ account ")
            .build();

        if let Some(uin) = payload.account {
            account.buffer().set_text(&uin.to_string())
        }

        let t_sender = sender.clone();
        account.connect_changed(move |entry| t_sender.send(Input::Account(entry.buffer().text())));

        let _pwd_row = ActionRow::builder()
            .title("Password")
            .focusable(false)
            .build();

        let pwd = PasswordEntry::builder()
            .valign(Align::Center)
            .placeholder_text("Please input your QQ password")
            .build();

        if let Some(ref p) = payload.password {
            pwd.set_text(p);
        }

        let t_sender = sender.clone();
        pwd.connect_changed(move |entry| t_sender.send(Input::Password(entry.text().to_string())));

        let login_btn = Button::builder().label("Log in")
        .visible(false)
        .build();

        let t_sender = sender.clone();
        login_btn.connect_clicked(move |_| t_sender.send(Input::Login));

        root.append(&avatar);

        root.append(&_group);

        _group.add(&_account_row);
        _account_row.add_suffix(&account);

        _group.add(&_pwd_row);
        _pwd_row.add_suffix(&pwd);

        root.append(&login_btn);

        Self {
            avatar,
            _group,
            _account_row,
            account,
            _pwd_row,
            pwd,
            login_btn,
        }
    }
}
