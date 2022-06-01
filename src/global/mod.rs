use adw::Application;
use gtk::{gio::Notification, prelude::ApplicationExt};
use once_cell::sync::OnceCell;
use relm4::{adw, gtk};

#[derive(Debug)]
pub struct SharedApplication {
    app: Application,
}

unsafe impl Sync for SharedApplication {}
unsafe impl Send for SharedApplication {}

impl SharedApplication {
    pub fn new(app: Application) -> Self {
        SharedApplication { app }
    }

    pub fn send_notification(&self, title: &String, body: &String) {
        let notification = Notification::new(&title);
        notification.set_body(Some(&body));
        self.app.send_notification(None, &notification);
    }
}

pub static APP: OnceCell<SharedApplication> = OnceCell::new();
