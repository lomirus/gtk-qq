use relm4::gtk::{self, gdk::Paintable, traits::EditableExt};

use super::{
    payloads::{Input, Output, Payload, PwdEntry, State},
    widgets::PwdLoginWidget,
};

#[derive(Debug)]
pub struct PasswordLoginModel {
    account_changed: bool,
    account_state: State,
    account: Option<i64>,
    password: PwdEntry,
    avatar: Option<Paintable>,
}

impl relm4::SimpleComponent for PasswordLoginModel {
    type Input = Input;

    type Output = Output;

    type InitParams = Payload;

    type Root = gtk::Box;

    type Widgets = PwdLoginWidget;

    fn init_root() -> Self::Root {
        gtk::Box::builder()
            .orientation(gtk::Orientation::Horizontal)
            .halign(gtk::Align::Center)
            .valign(gtk::Align::Center)
            .vexpand(true)
            .spacing(32)
            .build()
    }

    fn init(
        params: Self::InitParams,
        root: &Self::Root,
        sender: &relm4::ComponentSender<Self>,
    ) -> relm4::ComponentParts<Self> {
        sender.output(Output::EnableLogin(
            params.account.is_some() && params.token.is_some(),
        ));
        if params.auto_login {
            sender.input(Input::Login);
        }

        let widgets =
            PwdLoginWidget::new(root, &params, sender.input_sender(), sender.output_sender());

        let pwd = match params.token {
            Some(token) => PwdEntry::Token(token),
            None => PwdEntry::None,
        };

        let model = Self {
            account: params.account,
            password: pwd,
            account_changed: false,
            account_state: State::NoChange,
            avatar: params.avatar,
        };

        relm4::ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, sender: &relm4::ComponentSender<Self>) {
        match message {
            Input::Account(ac) => {
                if let State::NoChange = self.account_state {
                    if let Ok(uin) = ac.parse::<i64>() {
                        self.account.replace(uin);
                        self.account_changed = true;
                    } else if ac.is_empty() {
                        self.account = None;
                        self.account_changed = true;
                    } else {
                        self.account_state = State::Update;
                    }
                } else {
                    self.account_state = State::NoChange;
                }
            }
            Input::Password(pwd) => {
                if !pwd.is_empty() {
                    let n = match self.password {
                        PwdEntry::None => PwdEntry::Password(pwd),
                        PwdEntry::Token(_) => PwdEntry::None,
                        PwdEntry::Password(_) => PwdEntry::Password(pwd),
                    };
                    self.password = n;
                } else {
                    self.password = PwdEntry::None
                }
            }
            Input::Login => match (self.password.clone(), self.account) {
                (PwdEntry::Password(pwd), Some(account)) => {
                    sender.output(Output::Login { account, pwd })
                }
                (PwdEntry::Token(token), _) => sender.output(Output::TokenLogin(token)),
                (_, _) => sender.output(Output::EnableLogin(false)),
            },
            Input::Avatar(pic) => self.avatar = pic,
        }
    }

    fn update_view(&self, widgets: &mut Self::Widgets, sender: &relm4::ComponentSender<Self>) {
        if let State::Update = self.account_state {
            widgets.account.set_text(
                &self
                    .account
                    .map(|a| a.to_string())
                    .unwrap_or_else(String::new),
            );
        }

        if let PwdEntry::None = self.password {
            widgets._pwd.set_text("");
        }

        sender.output(Output::EnableLogin(
            self.account.is_some() && self.password.is_some(),
        ));

        if self.account_changed {
            widgets
                .avatar
                .set_custom_image(Option::<&'static Paintable>::None);
        }

        widgets
            .avatar
            .set_custom_image(Into::<Option<&Paintable>>::into(&self.avatar));
    }
}
