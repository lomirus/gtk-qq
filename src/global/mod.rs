use relm4::{adw, gtk};

use adw::{Application, ApplicationWindow};
use gtk::gdk_pixbuf::Pixbuf;
use gtk::gio::Notification;
use gtk::prelude::ApplicationExt;

use once_cell::sync::OnceCell;
use tokio::task;

use crate::db::{
    fs::{
        download_group_avatar_file, download_user_avatar_file, get_group_avatar_path,
        get_user_avatar_path,
    },
    sql::{get_friend_remark, get_group_name},
};

#[derive(Debug)]
pub struct SharedApplication {
    pub app: Application,
}

unsafe impl Sync for SharedApplication {}
unsafe impl Send for SharedApplication {}

impl SharedApplication {
    pub fn new(app: Application) -> Self {
        SharedApplication { app }
    }

    pub fn notify_friend_message(&self, friend_id: i64, content: &String) {
        let title = get_friend_remark(friend_id);
        let path = get_user_avatar_path(friend_id);

        let notification = Notification::new(&title);
        notification.set_body(Some(content));

        if path.exists() {
            if let Ok(icon) = Pixbuf::from_file(path) {
                notification.set_icon(&icon);
            }
        } else {
            task::spawn(download_user_avatar_file(friend_id));
        }

        self.app.send_notification(None, &notification);
    }

    pub fn notify_group_message(&self, group_id: i64, content: &String) {
        let title = get_group_name(group_id);
        let path = get_group_avatar_path(group_id);

        let notification = Notification::new(&title);
        notification.set_body(Some(content));

        if path.exists() {
            if let Ok(icon) = Pixbuf::from_file(path) {
                notification.set_icon(&icon);
            }
        } else {
            task::spawn(download_group_avatar_file(group_id));
        }

        self.app.send_notification(None, &notification);
    }
}

pub static APP: OnceCell<SharedApplication> = OnceCell::new();

#[derive(Debug)]
pub struct SharedWindow {
    pub window: ApplicationWindow,
}

unsafe impl Sync for SharedWindow {}
unsafe impl Send for SharedWindow {}

impl SharedWindow {
    pub fn new(window: ApplicationWindow) -> Self {
        SharedWindow { window }
    }
}

pub static WINDOW: OnceCell<SharedWindow> = OnceCell::new();
