use std::path::Path;

use relm4::{
    adw::Window,
    gtk::{self, gdk_pixbuf::Pixbuf, traits::GtkWindowExt},
    ComponentParts, ComponentSender,
};

use super::{
    background::qr_code_handler,
    payloads::{self, Input, Output},
    widgets::QrCodeLoginWidgets,
};

pub struct QrCodeLoginModel {
    picture: Option<Pixbuf>,
    temp_path: &'static Path,
    widows: Window,
    task_handle: tokio::task::JoinHandle<()>,
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
        sender: &ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let widget = QrCodeLoginWidgets::new(root);

        let handle = tokio::spawn(qr_code_handler(
            params.client,
            sender.input_sender().clone(),
            params.temp_img_path,
        ));

        ComponentParts {
            model: Self {
                picture: None,
                temp_path: params.temp_img_path,
                widows: params.windows,
                task_handle: handle,
            },
            widgets: widget,
        }
    }

    fn update(&mut self, message: Self::Input, sender: &ComponentSender<Self>) {
        match message {
            payloads::Input::UpdateQrCode => {
                let qrcode = Pixbuf::from_file(self.temp_path).expect("Error to load QrCode");
                self.picture.replace(qrcode);
            }
            payloads::Input::FollowLogin(login_resp) => {
                sender.output(Output::LoginGoAhead(login_resp));
                self.widows.close();
            }
            payloads::Input::Error(err) => {
                sender.output(Output::Error(err));
                self.widows.close();
            }
            payloads::Input::Updated => {
                self.picture.take();
            }
        }
    }

    fn update_view(&self, widgets: &mut Self::Widgets, sender: &ComponentSender<Self>) {
        if let Some(_) = self.picture {
            widgets
                .qr_code
                .set_pixbuf(Into::<Option<&Pixbuf>>::into(&self.picture));
            sender.input(Input::Updated);
        }
    }

    fn shutdown(&mut self, _: &mut Self::Widgets, _: relm4::Sender<Self::Output>) {
        self.task_handle.abort()
    }
}
