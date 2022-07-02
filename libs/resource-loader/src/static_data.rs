//! static data

use std::{io, path::PathBuf};

use once_cell::sync::OnceCell;

use crate::{
    configs::{Config, InnerConfig},
    utils::resource_root,
};

static CONFIGURATION: OnceCell<InnerConfig> = OnceCell::new();

pub struct ResourceConfig;

pub(crate) fn load_cfg() -> &'static InnerConfig {
    CONFIGURATION.get_or_init(|| {
        #[cfg(feature = "logger")]
        log::warn!("Config not set. Using Default Config");
        Config::default().into_inner()
    })
}

fn create_and_get_config_path() -> io::Result<PathBuf> {
    let res_root = resource_root();
    std::fs::create_dir_all(&res_root)?;
    Ok(res_root.join("config.toml"))
}

fn get_config_path() -> PathBuf {
    resource_root().join("config.toml")
}

impl ResourceConfig {
    pub fn set_config(cfg: Config) {
        if let Err(_) = CONFIGURATION.set(cfg.into_inner()) {
            panic!("Config had been set")
        }
    }

    pub fn load_or_create_default() -> io::Result<()> {
        let cfg_file = get_config_path();
        if !cfg_file.exists() || !cfg_file.is_file() {
            Self::save_config(Default::default())?;
            Self::set_config(Default::default());
            Ok(())
        } else {
            Self::load_from_file()
        }
    }

    pub fn load_from_file() -> io::Result<()> {
        let config_file = create_and_get_config_path()?;
        let file = std::fs::read(config_file)?;

        let cfg = toml::from_slice::<Config>(&file).expect("Bad Config File Format");

        Self::set_config(cfg);

        Ok(())
    }

    pub fn save_config(cfg: Config) -> io::Result<()> {
        let config_file = create_and_get_config_path()?;

        let cfg = toml::to_string_pretty(&cfg).expect("Serde Config Error");

        std::fs::write(config_file, cfg)?;

        Ok(())
    }
}
#[cfg(test)]
mod test {
    use crate::{ops::avatar, Config, GetPath};

    use super::ResourceConfig;

    #[test]
    fn generate_conf() {
        let cfg = Config::default();

        let cfg = toml::to_string_pretty(&cfg).unwrap();

        println!("{cfg}")
    }

    #[test]
    fn save_cfg() {
        let cfg = Config::default();

        ResourceConfig::save_config(cfg).unwrap();
    }
    #[test]
    fn load_cfg() {
        ResourceConfig::load_from_file().unwrap();

        let avatar_group = avatar::Group::get_path();
        println!("{avatar_group:?}")
    }
}
