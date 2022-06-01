#![feature(async_closure)]

mod actions;
mod app;
mod config;
mod db;
mod global;
mod handler;

use db::init_sqlite;
use gio::{resources_register, Cancellable, Resource};
use global::{SharedApplication, APP};
use gtk::{gio, glib::Bytes, prelude::ApplicationExt};
use relm4::{gtk, RelmApp};

use app::AppModel;

#[tokio::main]
async fn main() {
    init_resources();
    init_sqlite();

    let app: RelmApp<AppModel> = RelmApp::new(config::APPLICATION_ID);
    app.app.register(Option::<&Cancellable>::None).unwrap();
    relm4::set_global_css(include_bytes!("styles/style.css"));

    let shared_app = SharedApplication::new(app.app.clone());
    APP.set(shared_app).unwrap();

    app.run(());
}

fn init_resources() {
    let res_bytes = Bytes::from(include_bytes!("../builddir/assets/resources.gresource"));
    let res = Resource::from_data(&res_bytes).unwrap();
    resources_register(&res);
}
