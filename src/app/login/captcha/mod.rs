use relm4::gtk::glib::clone;
use relm4::gtk::Align;
use relm4::{adw, gtk, Component, ComponentController, SimpleComponent, WidgetPlus};

use adw::{Window, HeaderBar};
use gtk::prelude::*;
use gtk::{Box, Button, Entry, Label, Orientation, Picture};

use typed_builder::TypedBuilder;
use widgets::link_copier::{self, LinkCopierModel};

pub struct CaptchaModel;

pub enum Output {
    Submit { ticket: String },
    CopyLink,
}

#[derive(TypedBuilder)]
pub struct PayLoad {
    pub(crate) window: Window,
    pub(crate) verify_url: String,
    #[builder(default = String::from("https://github.com/mzdluo123/TxCaptchaHelper"))]
    pub(crate) scanner_url: String,
}


impl SimpleComponent for CaptchaModel {
    type Input = ();
    type Output = Output;
    type InitParams = PayLoad;
    type Root = Box;
    type Widgets = ();

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
                    .url(params.scanner_url)
                    .build(),
            )
            .forward(&sender.output, |msg| match msg {
                link_copier::Output::LinkCopied => Output::CopyLink,
            });

        let verify_link = LinkCopierModel::builder()
            .launch(
                link_copier::Payload::builder()
                    .url(params.verify_url)
                    .label("Verification Link".into())
                    .build(),
            )
            .forward(&sender.output, |msg| match msg {
                link_copier::Output::LinkCopied => Output::CopyLink,
            });

        let cloned_window = params.window.clone();

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
                            connect_activate[sender] => move |entry|{
                                sender.output(Output::Submit {
                                    ticket: entry.buffer().text(),
                                });
                                cloned_window.close();
                            }
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
                    set_filename: Some(&{
                        let mut path = dirs::home_dir().unwrap();
                        path.push(".gtk-qq");
                        path.push("captcha_url.png");
                        path
                    })
                }
            }
        }

        ticket_submit_button.connect_clicked(clone!(@strong sender => move |_| {
            sender.output(Output::Submit {
                ticket: ticket_input.buffer().text(),
            });
            params.window.close();
        }));

        root.append(&body);

        relm4::ComponentParts {
            model: Self,
            widgets: (),
        }
    }
}
