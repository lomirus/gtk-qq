#![feature(async_closure)]

mod actions;
mod app;
mod config;
mod handler;

use gtk::{gio, glib::Bytes};
use relm4::{gtk, RelmApp};

use app::AppModel;

#[tokio::main]
async fn main() {
    let bytes = Bytes::from(include_bytes!("../builddir/assets/resources.gresource"));
    let res = gio::Resource::from_data(&bytes).unwrap();
    gio::resources_register(&res);

    let app: RelmApp<AppModel> = RelmApp::new(config::APPLICATION_ID);
    relm4::set_global_css(include_bytes!("styles/style.css"));

    app.run(());
}
