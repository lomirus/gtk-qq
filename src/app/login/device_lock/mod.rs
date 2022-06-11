use relm4::{adw, gtk, Component, ComponentController, SimpleComponent, WidgetPlus};

use adw::{HeaderBar, Window};
use gtk::prelude::*;
use gtk::{Align, Box, Button, Label, Orientation};

use widgets::link_copier::{self, LinkCopierModel};

use super::LoginPageMsg;

pub struct DeviceLock;

#[derive(Debug)]
pub struct Payload {
    pub(crate) window: Window,
    pub(crate) unlock_url: String,
    pub(crate) sms_phone: Option<String>,
}

impl SimpleComponent for DeviceLock {
    type Input = ();
    type Output = LoginPageMsg;
    type InitParams = Payload;
    type Root = Box;
    type Widgets = ();

    fn init_root() -> Self::Root {
        relm4::view! {
            #[root]
            #[name = "root"]
            Box {
                set_orientation: Orientation::Vertical,
                HeaderBar {
                    set_title_widget = Some(&Label) {
                        set_label: "Device Lock Verify Introduction"
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
        relm4::view! {
            body = Box {
                set_orientation: Orientation::Vertical,
                set_valign: Align::Center,
                set_halign: Align::Center,
                set_vexpand: true,
                set_spacing: 24,
                set_margin_all: 16,
                Label {
                    set_label: &format!(
                        "Please open the link below and use your logged in device[sms:{}] to verify",
                        params.sms_phone.unwrap_or_else(|| "<unknown>".into())
                    )
                },
                append: LinkCopierModel::builder()
                    .launch(
                        link_copier::Payload::builder()
                            .url(params.unlock_url)
                            .label("Device Lock Verification".into())
                            .build(),
                    )
                    .forward(sender.output_sender(), |msg| match msg {
                        link_copier::Output::LinkCopied => LoginPageMsg::LinkCopied,
                    })
                    .widget(),
                Label {
                    set_label: "Once verified, click the button below"
                },
                Button {
                    set_label: "Confirm Verification",
                    connect_clicked[sender] => move |_| {
                        sender.output(LoginPageMsg::ConfirmVerification);
                        params.window.close();
                    },
                }
            }
        }

        root.append(&body);

        relm4::ComponentParts {
            model: Self,
            widgets: (),
        }
    }
}
