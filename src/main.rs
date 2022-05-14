mod app;
mod config;
mod pages;

use gtk::gio;
use relm4::{adw, gtk, RelmApp};

use app::AppModel;

fn main() {
    let res = gio::Resource::load(config::PKGDATA_DIR.to_owned() + "/resources.gresource")
        .expect("Could not load resources");
    gio::resources_register(&res);

    let application = adw::Application::builder()
        .application_id(config::APPLICATION_ID)
        .build();

    let model = AppModel::new();
    let app = RelmApp::with_app(model, application);
    relm4::set_global_css(include_bytes!("styles/style.css"));
    
    app.run()
}
