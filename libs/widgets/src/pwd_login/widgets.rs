use relm4::{
    adw::{
        traits::{ActionRowExt, PreferencesGroupExt},
        ActionRow, Avatar, PreferencesGroup,
    },
    gtk::{
        self,
        prelude::EntryBufferExtManual,
        traits::{BoxExt, ButtonExt, EditableExt, EntryExt},
        Align, Button, CheckButton, Entry, EntryBuffer, PasswordEntry,
    },
    Sender,
};

use super::payloads::{Input, Payload};
#[derive(Debug)]
pub struct PwdLoginWidget {
    _input_area: gtk::Box,
    pub(super) avatar: Avatar,
    _group: PreferencesGroup,
    _account_row: ActionRow,
    pub(super) account: Entry,
    _pwd_row: ActionRow,
    pub(super) _pwd: PasswordEntry,
    _op_box: gtk::Box,
    _cfg_box: gtk::Box,
    pub(super) _remember_pwd: CheckButton,
    pub(super) _auto_login: CheckButton,

    pub(super) login_btn: Button,
}

impl PwdLoginWidget {
    pub(super) fn new(root: &gtk::Box, payload: &Payload, sender: &Sender<Input>) -> Self {
        let input_area = gtk::Box::builder()
            .orientation(gtk::Orientation::Horizontal)
            .halign(gtk::Align::Center)
            .valign(gtk::Align::Center)
            .vexpand(true)
            .spacing(32)
            .build();

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
            .placeholder_text("QQ account")
            .build();

        if let Some(uin) = payload.account {
            let buf = EntryBuffer::new(Some(&uin.to_string()));
            account.set_buffer(&buf);
        }

        let t_sender = sender.clone();
        account.connect_changed(move |entry| t_sender.send(Input::Account(entry.buffer().text())));

        let _pwd_row = ActionRow::builder()
            .title("Password")
            .focusable(false)
            .build();

        let pwd = PasswordEntry::builder()
            .valign(Align::Center)
            .show_peek_icon(true)
            .placeholder_text("QQ password")
            .build();

        if let Some(ref p) = payload.password {
            pwd.set_text(p);
        }
        let t_sender = sender.clone();
        pwd.connect_changed(move |entry| t_sender.send(Input::Password(entry.text().to_string())));

        let op_box = gtk::Box::builder()
            .orientation(gtk::Orientation::Horizontal)
            .halign(Align::End)
            .spacing(24)
            .vexpand(true)
            .build();

        let cfg_box = gtk::Box::builder()
            .orientation(gtk::Orientation::Horizontal)
            .valign(Align::Center)
            .halign(Align::Center)
            .spacing(8)
            .build();

        let remember_pwd = gtk::CheckButton::builder()
            .label("remember password")
            .sensitive(false)
            .build();

        let auto_login = gtk::CheckButton::builder()
            .label("auto login")
            .sensitive(false)
            .build();

        let login_btn = Button::builder()
            .icon_name(payload.icon_name)
            .hexpand_set(true)
            .sensitive(payload.account.is_some() && payload.password.is_some())
            .build();

        let t_sender = sender.clone();
        login_btn.connect_clicked(move |_| t_sender.send(Input::Login));

        root.append(&input_area);
        input_area.append(&avatar);

        input_area.append(&_group);

        _group.add(&_account_row);
        _account_row.add_suffix(&account);

        _group.add(&_pwd_row);
        _pwd_row.add_suffix(&pwd);

        root.append(&op_box);
        op_box.append(&cfg_box);
        cfg_box.append(&remember_pwd);
        cfg_box.append(&auto_login);
        op_box.append(&login_btn);

        Self {
            avatar,
            _group,
            _account_row,
            account,
            _pwd_row,
            _pwd: pwd,
            login_btn,
            _input_area: input_area,
            _op_box: op_box,
            _cfg_box: cfg_box,
            _remember_pwd: remember_pwd,
            _auto_login: auto_login,
        }
    }
}
