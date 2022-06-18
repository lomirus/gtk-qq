//! storage static data

use state::Storage;

use crate::model::{Config, InnerConfig};

static CONFIGURATION: Storage<InnerConfig> = Storage::new();

fn load_cfg() -> &'static InnerConfig {
    if let Some(cfg) = CONFIGURATION.try_get() {
        cfg
    } else {
        panic!("config not init yet")
    }
}

fn set_config(cfg: Config) {
    unimplemented!()
}

fn update_config(cfg: Config) {
    unimplemented!()
}
