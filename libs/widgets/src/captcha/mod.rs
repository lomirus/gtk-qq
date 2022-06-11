use relm4::{
    gtk::{self, Orientation},
    Component, ComponentController, SimpleComponent,
};

use crate::link_copier::{self, LinkCopier, LinkCopierModel};

mod payloads;
mod widget;

pub use payloads::{Output, PayLoad};
pub use widget::CaptchaWidgets;

pub struct CaptchaModel {
    _scanner_link: LinkCopier,
    _verify_link: LinkCopier,
}

impl SimpleComponent for CaptchaModel {
    type Input = ();

    type Output = Output;

    type InitParams = PayLoad;

    type Root = gtk::Box;

    type Widgets = CaptchaWidgets;

    fn init_root() -> Self::Root {
        gtk::Box::builder()
            .orientation(Orientation::Vertical)
            .build()
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
                    .label("Verification link".into())
                    .build(),
            )
            .forward(&sender.output, |msg| match msg {
                link_copier::Output::LinkCopied => Output::CopyLink,
            });

        let widgets = CaptchaWidgets::new(
            root,
            scanner_link.widget(),
            verify_link.widget(),
            &params.window,
            sender,
        );

        let model = Self {
            _scanner_link: scanner_link,
            _verify_link: verify_link,
        };
        relm4::ComponentParts { model, widgets }
    }
}
