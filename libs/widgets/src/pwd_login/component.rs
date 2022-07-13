use relm4::gtk::{
    self,
    gdk::Paintable,
    traits::{EditableExt, WidgetExt},
};

use super::{
    payloads::{Input, Output, Payload, State},
    widgets::PwdLoginWidget,
};

#[derive(Debug)]
pub struct PasswordLoginModel {
    account_changed: bool,
    account_state: State,
    account: Option<i64>,
    password: Option<String>,
}

impl relm4::SimpleComponent for PasswordLoginModel {
    type Input = Input;

    type Output = Output;

    type InitParams = Payload;

    type Root = gtk::Box;

    type Widgets = PwdLoginWidget;

    fn init_root() -> Self::Root {
        gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .halign(gtk::Align::Center)
            .valign(gtk::Align::Center)
            .vexpand(true)
            .spacing(10)
            .build()
    }

    fn init(
        params: Self::InitParams,
        root: &Self::Root,
        sender: &relm4::ComponentSender<Self>,
    ) -> relm4::ComponentParts<Self> {
        let widgets = PwdLoginWidget::new(root, &params, sender.input_sender());
        let model = Self {
            account: params.account,
            password: params.password,
            account_changed: false,
            account_state: State::NoChange,
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
                    self.password.replace(pwd);
                } else {
                    self.password = None
                }
            }
            Input::Login => sender.output(Output::Login {
                account: self.account.unwrap(),
                pwd: self.password.clone().unwrap(),
            }),
            Input::Avatar(_) => todo!(),
        }
    }

    fn update_view(&self, widgets: &mut Self::Widgets, _sender: &relm4::ComponentSender<Self>) {
        if let State::Update = self.account_state {
            widgets.account.set_text(
                &self
                    .account
                    .map(|a| a.to_string())
                    .unwrap_or_else(String::new),
            );
        }

        widgets
            .login_btn
            .set_sensitive(self.account.is_some() && self.password.is_some());

        if self.account_changed {
            widgets
                .avatar
                .set_custom_image(Option::<&'static Paintable>::None);
        }
    }
}
