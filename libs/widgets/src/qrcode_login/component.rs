use std::{cell::RefCell, path::Path};

use relm4::{
    gtk::{self, gdk_pixbuf::Pixbuf},
    ComponentParts, ComponentSender,
};

use super::{
    payloads::{self},
    widgets::QrCodeLoginWidgets,
};

#[derive(Debug)]
pub struct QrCodeLoginModel {
    picture: RefCell<Option<Pixbuf>>,
    temp_path: &'static Path,
}

impl relm4::SimpleComponent for QrCodeLoginModel {
    type Input = payloads::Input;

    type Output = payloads::Output;

    type InitParams = payloads::PayLoad;

    type Root = gtk::Box;

    type Widgets = QrCodeLoginWidgets;

    fn init_root() -> Self::Root {
        gtk::Box::builder()
            .width_request(400)
            .height_request(300)
            .spacing(5)
            .build()
    }

    fn init(
        params: Self::InitParams,
        root: &Self::Root,
        _: &ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let widget = QrCodeLoginWidgets::new(root);
        ComponentParts {
            model: Self {
                picture: RefCell::new(None),
                temp_path: params.temp_img_path,
            },
            widgets: widget,
        }
    }

    fn update(&mut self, message: Self::Input, _: &ComponentSender<Self>) {
        match message {
            payloads::Input::UpdateQrCode => {
                self.picture
                    .borrow_mut()
                    .replace(Pixbuf::from_file(self.temp_path).expect("Error to load QrCode"));
            }
        }
    }

    fn update_view(&self, widgets: &mut Self::Widgets, _: &ComponentSender<Self>) {
        if let Some(pic) = self.picture.borrow_mut().take() {
            widgets.qr_code.set_pixbuf(Some(&pic));
        }
    }
}
