use relm4::{
    gtk::{
        self,
        traits::{ButtonExt, WidgetExt},
        Button, LinkButton,
    },
    ComponentSender,
};

use super::{payloads::Payload, LinkCopierModel, Output};

#[derive(Debug, typed_builder::TypedBuilder)]
pub struct LinkCopierWidgets {
    pub(super) link_btn: gtk::LinkButton,
    pub(super) copy_btn: gtk::Button,
}

impl LinkCopierWidgets {
    pub(super) fn new(cfg: &Payload, sender: ComponentSender<LinkCopierModel>) -> Self {
        let label: Option<&String> = (&cfg.label).into();
        let label = label.map(String::as_str).unwrap_or(&cfg.url);

        let link_btn = Self::create_link_btn(&cfg.url, &label);
        let copy_btn = Self::create_copy_btn();

        let url = cfg.url.clone();
        copy_btn.connect_clicked(move |btn| {
            // past url to clipboard
            let clipboard = btn.clipboard();
            clipboard.set_text(&url);

            sender.output(Output::LinkCopied);
        });

        Self { link_btn, copy_btn }
    }

    fn create_link_btn(uri: &impl AsRef<str>, label: &impl AsRef<str>) -> LinkButton {
        gtk::LinkButton::builder()
            // .css_name("link-part")
            .uri(uri.as_ref())
            .label(label.as_ref())
            .build()
    }

    fn create_copy_btn() -> Button {
        gtk::Button::builder()
            // .css_name("copy-part")
            .label("Copy")
            .build()
    }
}
