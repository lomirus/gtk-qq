// Delete this after migration
#![allow(unused_variables)]

mod actions;
mod app;
mod config;
mod pages;

use gtk::gio;
use relm4::{gtk, RelmApp};

use app::AppModel;

fn main() {
    let res = gio::Resource::load(config::PKGDATA_DIR.to_owned() + "/resources.gresource")
        .expect("Could not load resources");
    gio::resources_register(&res);

    let app: RelmApp<AppModel> = RelmApp::new(config::APPLICATION_ID);
    relm4::set_global_css(include_bytes!("styles/style.css"));

    app.run(());
}
