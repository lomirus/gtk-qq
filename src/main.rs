#![feature(async_closure)]

mod actions;
mod app;
mod config;
mod handler;

use adw::Application;
use gtk::{
    gio::{self, Cancellable},
    glib::Bytes,
    prelude::ApplicationExt,
};
use once_cell::sync::OnceCell;
use relm4::{adw, gtk, RelmApp};

use app::AppModel;

#[derive(Debug)]
struct SharedApplication {
    app: Application,
}

unsafe impl Sync for SharedApplication {}
unsafe impl Send for SharedApplication {}

static APP: OnceCell<SharedApplication> = OnceCell::new();

#[tokio::main]
async fn main() {
    let res_bytes = Bytes::from(include_bytes!("../builddir/assets/resources.gresource"));
    let res = gio::Resource::from_data(&res_bytes).unwrap();
    gio::resources_register(&res);

    let app: RelmApp<AppModel> = RelmApp::new(config::APPLICATION_ID);
    app.app.register(Option::<&Cancellable>::None).unwrap();
    relm4::set_global_css(include_bytes!("styles/style.css"));

    let shared_app = SharedApplication {
        app: app.app.clone(),
    };
    APP.set(shared_app).unwrap();

    app.run(());
}
