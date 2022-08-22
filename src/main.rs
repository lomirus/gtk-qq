#![feature(async_closure)]
#![feature(strict_provenance)]

mod actions;
mod app;
mod config;
mod db;
mod global;
mod handler;
mod utils;

use gio::{resources_register, Cancellable, Resource};
use gtk::{gio, glib::Bytes, prelude::ApplicationExt};
use log::Level;
use relm4::{gtk, RelmApp};

use app::AppModel;
use db::sql::init_sqlite;
use global::{SharedApplication, APP};
use resource_loader::ResourceConfig;
use yansi::Color;

#[tokio::main]
async fn main() {
    init_logger();
    ResourceConfig::load_or_create_default().expect("Failure on loading configuration");
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

fn init_logger() {
    //TODO: logger to file
    // init logger
    fern::Dispatch::new()
        .format(|out, message, record| {
            let log_color = yansi::Style::new(match record.level() {
                Level::Error => Color::Red,
                Level::Warn => Color::Yellow,
                Level::Info => Color::Green,
                Level::Debug => Color::Cyan,
                Level::Trace => Color::Magenta,
            })
            .bold();
            out.finish(format_args!(
                "[{level}]{time}[{model}:{line}] {message}",
                time = chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
                model = record.target(),
                line = record.line().unwrap_or(0),
                level = log_color.paint(record.level()),
            ))
        })
        .level(log::LevelFilter::Debug)
        .chain(std::io::stdout())
        .apply()
        .expect("Failure start logger");
}
