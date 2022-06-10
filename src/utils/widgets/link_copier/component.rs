use std::sync::Arc;

use relm4::{
    gtk::{
        self,
        traits::{BoxExt, ButtonExt, WidgetExt},
    },
    ComponentParts,
};

use super::{widgets::LinkCopierWidgets, Input, Output, Payload, State};

#[derive(Debug)]
pub struct LinkCopierModel {
    link: String,
    label: Option<String>,
    state: State,
}

impl relm4::SimpleComponent for LinkCopierModel {
    type Input = Input;

    type Output = Output;

    type InitParams = Payload;

    type Root = gtk::Box;

    type Widgets = LinkCopierWidgets;

    fn init_root() -> Self::Root {
        gtk::Box::builder()
            .css_name("link-copier")
            .orientation(gtk::Orientation::Horizontal)
            .halign(gtk::Align::Center)
            .valign(gtk::Align::Center)
            .spacing(3)
            .build()
    }

    fn init(
        params: Self::InitParams,
        root: &Self::Root,
        sender: &relm4::ComponentSender<Self>,
    ) -> relm4::ComponentParts<Self> {
        let ty = params.ty;
        let widget = Self::Widgets::new(&params, Arc::clone(sender));

        match ty {
            State::Both => {
                root.append(&widget.link_btn);
                root.append(&widget.copy_btn);
            }
            State::LinkOnly => root.append(&widget.link_btn),
            State::BtnOnly => root.append(&widget.copy_btn),
        };

        let model = LinkCopierModel {
            link: params.url,
            label: params.label,
            state: ty,
        };

        ComponentParts {
            model,
            widgets: widget,
        }
    }

    fn update(&mut self, message: Self::Input, _sender: &relm4::ComponentSender<Self>) {
        match message {
            Input::Link(url) => self.link = url.into_owned(),
            Input::Label(label) => {
                self.label.replace(label.into_owned());
            }
            Input::State(s) => self.state = s,
        }
    }

    fn update_view(&self, widgets: &mut Self::Widgets, _sender: &relm4::ComponentSender<Self>) {
        let label: Option<_> = (&self.label).into();
        let label = label.unwrap_or(&self.link).as_str();

        widgets.link_btn.set_uri(&self.link);
        widgets.link_btn.set_label(label);

        match self.state {
            State::Both => {
                widgets.copy_btn.set_visible(true);
                widgets.link_btn.set_visible(true);
            }
            State::LinkOnly => {
                widgets.copy_btn.set_visible(false);
                widgets.link_btn.set_visible(true);
            }
            State::BtnOnly => {
                widgets.copy_btn.set_visible(true);
                widgets.link_btn.set_visible(false);
            }
        }
    }
}
