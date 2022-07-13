use relm4::gtk::{
    self,
    gdk::Paintable,
    traits::{EditableExt, WidgetExt},
};

use super::{
    payloads::{Input, Output, Payload},
    widgets::PwdLoginWidget,
};

pub struct PasswordLoginModel {
    account_changed: bool,
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
        let widgets = PwdLoginWidget::new(root, &params, sender.input_sender());
        let model = Self {
            account: params.account,
            password: params.password,
            account_changed: false,
        };

        relm4::ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, sender: &relm4::ComponentSender<Self>) {
        match message {
            Input::Account(ac) => {
                if let Ok(uin) = ac.parse::<i64>() {
                    self.account.replace(uin);
                    self.account_changed = true;
                }
            }
            Input::Password(pwd) => {
                self.password.replace(pwd);
            }
            Input::Login => sender.output(Output::Login {
                account: self.account.unwrap(),
                pwd: self.password.clone().unwrap(),
            }),
        }
    }

    fn update_view(&self, widgets: &mut Self::Widgets, _sender: &relm4::ComponentSender<Self>) {
        widgets.account.set_text(
            &self
                .account
                .map(|a| a.to_string())
                .unwrap_or_else(String::new),
        );

        widgets.pwd.set_text(
            Into::<Option<&String>>::into(&self.password)
                .map(|s| s.as_str())
                .unwrap_or(""),
        );

        if self.account.is_some() && self.password.is_some() {
            widgets.login_btn.set_visible(true);
        }

        if self.account_changed {
            widgets
                .avatar
                .set_custom_image(Option::<&'static Paintable>::None);
        }
    }
}
