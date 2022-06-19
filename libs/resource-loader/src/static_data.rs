//! storage static data

use std::{io, path::PathBuf};

use state::Storage;

use crate::{
    configs::{Config, InnerConfig},
    utils::resource_root,
};

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

fn get_and_create_config() -> io::Result<PathBuf> {
    let res_root = resource_root();
    std::fs::create_dir_all(&res_root)?;
    Ok(res_root.join("config.toml"))
}

pub fn load_from_file() -> io::Result<()> {
    let config_file = get_and_create_config()?;
    let file = std::fs::read(config_file)?;

    let cfg = toml::from_slice::<Config>(&file).expect("Bad Config File Format");

    set_config(cfg);

    Ok(())
}

#[allow(dead_code)]
pub fn save_config(cfg: Config) -> io::Result<()> {
    let config_file = get_and_create_config()?;

    let cfg = toml::to_string_pretty(&cfg).expect("Serde Config Error");

    std::fs::write(config_file, cfg)?;

    Ok(())
}

#[cfg(test)]
mod test {
    use crate::{ops::avatar, Config, GetPath};

    use super::{load_from_file, save_config};

    #[test]
    fn generate_conf() {
        let cfg = Config::default();

        let cfg = toml::to_string_pretty(&cfg).unwrap();

        println!("{cfg}")
    }

    #[test]
    fn save_cfg() {
        let cfg = Config::default();

        save_config(cfg).unwrap();
    }
    #[test]
    fn load_cfg() {
        load_from_file().unwrap();

        let avatar_group = avatar::Group::get_path();
        println!("{avatar_group:?}")
    }
}
