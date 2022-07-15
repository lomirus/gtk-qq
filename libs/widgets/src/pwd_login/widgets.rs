use relm4::{
    adw::{
        traits::{ActionRowExt, PreferencesGroupExt},
        ActionRow, Avatar, PreferencesGroup,
    },
    gtk::{
        self,
        prelude::EntryBufferExtManual,
        traits::{BoxExt, CheckButtonExt, EditableExt, EntryExt},
        Align, CheckButton, Entry, EntryBuffer, PasswordEntry,
    },
    Sender,
};

use super::{
    payloads::{Input, Payload},
    Output,
};
#[derive(Debug)]
pub struct PwdLoginWidget {
    _input_area: gtk::Box,
    pub(super) avatar: Avatar,
    _group: PreferencesGroup,
    _account_row: ActionRow,
    pub(super) account: Entry,
    _pwd_row: ActionRow,
    pub(super) _pwd: PasswordEntry,
    _cfg_box: gtk::Box,
    pub(super) _remember_pwd: CheckButton,
    pub(super) _auto_login: CheckButton,
}

impl PwdLoginWidget {
    pub(super) fn new(
        root: &gtk::Box,
        payload: &Payload,
        sender: &Sender<Input>,
        output: &Sender<Output>,
    ) -> Self {
        let input_area = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .halign(gtk::Align::Center)
            .valign(gtk::Align::Center)
            .vexpand(true)
            .spacing(12)
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
            .activates_default(true)
            .placeholder_text("QQ password")
            .build();

        if payload.token.is_some() {
            pwd.set_text("0123456789");
        }
        let t_sender = sender.clone();
        pwd.connect_changed(move |entry| t_sender.send(Input::Password(entry.text().to_string())));

        let t_sender = sender.clone();
        pwd.connect_activate(move |_| t_sender.send(Input::Login));

        let edit_box = gtk::Box::builder()
            .orientation(gtk::Orientation::Horizontal)
            .valign(Align::Center)
            .halign(Align::Center)
            .spacing(8)
            .build();

        let remember_pwd = gtk::CheckButton::builder()
            .active(payload.remember_pwd)
            .label("Remember Password")
            .build();

        let t_sender = output.clone();
        remember_pwd.connect_toggled(move |this| {
            t_sender.send(Output::RememberPwd(this.is_active()));
        });

        let auto_login = gtk::CheckButton::builder()
            .label("Auto Login")
            .sensitive(false)
            .active(payload.auto_login)
            .build();

        let t_sender = output.clone();
        auto_login.connect_toggled(move |this| {
            t_sender.send(Output::AutoLogin(this.is_active()));
        });

        root.append(&avatar);
        root.append(&input_area);

        input_area.append(&_group);
        
        _group.add(&_account_row);
        _account_row.add_suffix(&account);
        
        _group.add(&_pwd_row);
        _pwd_row.add_suffix(&pwd);
        
        input_area.append(&edit_box);

        edit_box.append(&remember_pwd);
        edit_box.append(&auto_login);

        Self {
            avatar,
            _group,
            _account_row,
            account,
            _pwd_row,
            _pwd: pwd,
            _input_area: input_area,
            _cfg_box: edit_box,
            _remember_pwd: remember_pwd,
            _auto_login: auto_login,
        }
    }
}
