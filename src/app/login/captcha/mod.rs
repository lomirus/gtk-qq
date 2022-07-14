use std::sync::Arc;

use relm4::gtk::glib::clone;
use relm4::gtk::Align;
use relm4::{adw, gtk, Component, ComponentController, ComponentSender, WidgetPlus};

use adw::{HeaderBar, Window};
use gtk::prelude::*;
use gtk::{Box, Button, Entry, Label, Orientation, Picture};

use resource_loader::{CaptchaQrCode, GetPath};
use ricq::Client;
use tokio::task;
use widgets::link_copier::{self, LinkCopierModel};

use super::LoginPageMsg;

#[derive(Clone)]
pub struct CaptchaModel {
    pub(crate) client: Arc<Client>,
    pub(crate) ticket: String,
}

pub struct CaptchaWidgets {
    window: Window,
}

pub enum Input {
    UpdateTicket(String),
    Submit,
    CloseWindow,
}

pub struct PayLoad {
    pub(crate) client: Arc<Client>,
    pub(crate) window: Window,
    pub(crate) verify_url: String,
}

impl Component for CaptchaModel {
    type Input = Input;
    type Output = LoginPageMsg;
    type InitParams = PayLoad;
    type Root = Box;
    type Widgets = CaptchaWidgets;
    type CommandOutput = ();

    fn init_root() -> Self::Root {
        relm4::view! {
            root = Box {
                set_orientation: Orientation::Vertical,
                HeaderBar {
                    set_title_widget = Some(&Label) {
                        set_label: "Captcha Verify Introduction"
                    }
                }
            }
        }
        root
    }

    fn init(
        params: Self::InitParams,
        root: &Self::Root,
        sender: &relm4::ComponentSender<Self>,
    ) -> relm4::ComponentParts<Self> {
        let scanner_link = LinkCopierModel::builder()
            .launch(
                link_copier::Payload::builder()
                    .url("https://github.com/mzdluo123/TxCaptchaHelper".to_string())
                    .build(),
            )
            .forward(&sender.output, |msg| match msg {
                link_copier::Output::LinkCopied => LoginPageMsg::LinkCopied,
            });

        let verify_link = LinkCopierModel::builder()
            .launch(
                link_copier::Payload::builder()
                    .url(params.verify_url.clone())
                    .label("Verification Link".into())
                    .build(),
            )
            .forward(&sender.output, |msg| match msg {
                link_copier::Output::LinkCopied => LoginPageMsg::LinkCopied,
            });

        relm4::view! {
            #[name = "body"]
            Box {
                set_valign: Align::Center,
                set_halign: Align::Center,
                set_vexpand: true,
                set_spacing: 24,
                Box {
                    set_margin_all: 16,
                    set_orientation: Orientation::Vertical,
                    set_halign: Align::Start,
                    set_spacing: 8,
                    Label {
                        set_xalign: 0.0,
                        set_label: "1. Install the tool on your Android phone: "
                    },
                    append: scanner_link.widget(),
                    Label {
                        set_xalign: 0.0,
                        set_label: "2. Scan the qrcode and get the ticket."
                    },
                    Box {
                        set_orientation: Orientation::Horizontal,
                        Label {
                            set_label: "3. "
                        },
                        #[name = "ticket_input"]
                        Entry {
                            set_placeholder_text: Some("Paste the ticket here..."),
                            set_margin_end: 8,
                            set_activates_default: true,
                            connect_activate[sender] => move |_|{
                                sender.input(Input::Submit);
                            },
                            connect_changed[sender] => move |entry|{
                                let ticket = entry.buffer().text();
                                sender.input(Input::UpdateTicket(ticket));
                            },
                        },
                        #[name = "ticket_submit_button"]
                        Button {
                            set_label: "Submit Ticket"
                        }
                    },
                    Box {
                        set_orientation: Orientation::Horizontal,
                        Label {
                            set_xalign: 0.0,
                            set_label: "Help: If you do not have an Android phone to install the tool, open the"
                        },
                        append: verify_link.widget(),
                        Label {
                            set_xalign: 0.0,
                            set_label: " in the browser manually, open the devtools and switch to the network panel. After you passed the"
                        },
                        Label {
                            set_xalign: 0.0,
                            set_label: "verification, you will find a request whose response contains the `ticket`. Then just paste it"
                        },
                        Label {
                            set_xalign: 0.0,
                            set_label: "above. The result would be same. It just maybe more complex if you don't know devtools well."
                        },
                    }
                },
                Picture {
                    set_width_request: 240,
                    set_can_shrink: true,
                    set_filename: Some(&CaptchaQrCode::get_path())
                }
            }
        }

        ticket_submit_button.connect_clicked(clone!(@strong sender => move |_| {
            sender.input(Input::Submit);
        }));

        root.append(&body);

        relm4::ComponentParts {
            model: CaptchaModel {
                client: params.client,
                ticket: String::new(),
            },
            widgets: CaptchaWidgets {
                window: params.window,
            },
        }
    }

    fn update_with_view(
        &mut self,
        widgets: &mut Self::Widgets,
        message: Self::Input,
        sender: &ComponentSender<Self>,
    ) {
        match message {
            Input::Submit => {
                task::spawn(self.clone().submit_ticket(sender.clone()));
            }
            Input::UpdateTicket(new_ticket) => {
                self.ticket = new_ticket;
            }
            Input::CloseWindow => {
                widgets.window.close();
            }
        }
    }
}

impl CaptchaModel {
    async fn submit_ticket(self, sender: ComponentSender<CaptchaModel>) {
        match self.client.submit_ticket(&self.ticket).await {
            Ok(res) => sender.output(LoginPageMsg::LoginRespond(res.into(), self.client.clone())),
            Err(err) => {
                sender.output(LoginPageMsg::LoginFailed(err.to_string()));
            }
        }
        sender.input(Input::CloseWindow);
    }
}
