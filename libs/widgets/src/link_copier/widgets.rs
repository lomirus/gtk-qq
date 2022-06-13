use relm4::{gtk, ComponentSender};

use gtk::prelude::*;
use gtk::{Button, LinkButton};

use super::{payloads::Payload, LinkCopierModel, Output};

#[derive(Debug, typed_builder::TypedBuilder)]
pub struct LinkCopierWidgets {
    pub(super) link_btn: LinkButton,
    pub(super) copy_btn: Button,
}

impl LinkCopierWidgets {
    pub(super) fn new(cfg: &Payload, sender: ComponentSender<LinkCopierModel>) -> Self {
        let link_btn = Self::create_link_btn(cfg.label.clone(), cfg.url.clone());
        let copy_btn = Self::create_copy_btn(cfg.url.clone(), sender);

        Self { link_btn, copy_btn }
    }

    fn create_link_btn(label: Option<String>, url: String) -> LinkButton {
        let label = label.unwrap_or_else(|| url.clone());
        LinkButton::builder().uri(&url).label(&label).build()
    }

    fn create_copy_btn(url: String, sender: ComponentSender<LinkCopierModel>) -> Button {
        let button = Button::builder().label("Copy Link").build();
        button.connect_clicked(move |btn| {
            // Paste the url to clipboard
            let clipboard = btn.clipboard();
            clipboard.set_text(&url);

            sender.output(Output::LinkCopied);
        });
        button
    }
}
