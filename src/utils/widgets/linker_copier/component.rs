use relm4::{gtk::{self, Box, traits::BoxExt}, Component, adw::{Toast, ToastOverlay}, ComponentParts};

use crate::utils::widgets::CustomWidget;

use super::{builder::{LinkerCopierCfg, LinkCopierState}, widgets::LinkCopierWidgets};

pub struct LinkCopierModel;


impl relm4::SimpleComponent for LinkCopierModel {
    type Input = ();

    type Output = ();

    type InitParams = LinkerCopierCfg;

    type Root = gtk::Box;

    type Widgets = LinkCopierWidgets;

    fn init_root() -> Self::Root {
        <Self as CustomWidget>::init_root()
    }

    fn init(
        params: Self::InitParams,
        root: &Self::Root,
        sender: &relm4::ComponentSender<Self>,
    ) -> relm4::ComponentParts<Self> {
        let widget = <Self as CustomWidget>::init(params, root);


        ComponentParts{ model: LinkCopierModel, widgets: widget }
    }
}

impl CustomWidget for LinkCopierModel {
    type Root=gtk::Box;

    type InitParams=LinkerCopierCfg;
    type Widgets = LinkCopierWidgets;
    fn init_root()->Self::Root {
        gtk::Box::builder()
        .css_name("link-copier")
            .orientation(gtk::Orientation::Horizontal)
            .halign(gtk::Align::Center)
            .valign(gtk::Align::Center)
            .spacing(3)
            .build()
    }

    fn init(params:Self::InitParams,root:&Self::Root) ->Self::Widgets{
        let ty = params.ty;
        let widget = Self::Widgets::new(params);

        match ty {
            LinkCopierState::Both => {
                root.append(&widget.link_btn);
                root.append(&widget.copy_btn);
            },
            LinkCopierState::LinkOnly => root.append(&widget.link_btn),
            LinkCopierState::BtnOnly => root.append(&widget.copy_btn),
        }

        widget
    }
}