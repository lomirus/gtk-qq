//! storage static data

use state::Storage;

use crate::configs::{Config, InnerConfig};

static CONFIGURATION: Storage<InnerConfig> = Storage::new();

pub(crate) fn load_cfg() -> &'static InnerConfig {
    CONFIGURATION.get_or_set(|| {
        #[cfg(feature = "logger")]
        log::warn!("Config not set, Using Default Config");
        Config::default().into_inner()
    })
}

pub fn set_config(cfg: Config) {
    if !CONFIGURATION.set(cfg.into_inner()) {
        panic!("Config had been set")
    }
}
