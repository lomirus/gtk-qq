use std::{
    io,
    path::{Path, PathBuf},
};

use tap::Tap;

use crate::logger;

pub struct ResourceDirectories {
    // under linux i864 x86_64 apple x86_64
    #[cfg(any(
        all(target_os = "linux", any(target_arch = "x86_64", target_arch = "i864")),
        all(target_os = "macos", target_arch = "x86_64",)
    ))]
    root_dir: xdg::BaseDirectories,
    #[cfg(not(any(
        all(target_os = "linux", any(target_arch = "x86_64", target_arch = "i864")),
        all(target_os = "macos", target_arch = "x86_64",)
    )))]
    root_dir: PathBuf,
}

#[cfg(not(any(
    all(target_os = "linux", any(target_arch = "x86_64", target_arch = "i864")),
    all(target_os = "macos", target_arch = "x86_64",)
)))]
impl ResourceDirectories {
    pub fn new_from(path: impl AsRef<Path>) -> Self {
        Self {
            root_dir: path.as_ref().to_path_buf(),
        }
    }

    pub fn new() -> Self {
        let root_dir = dirs::home_dir()
            .expect("User Home directory not exist")
            .join(".gtk-qq")
            .tap(|path| logger!(info "config local directory : {:?}", path));

        Self { root_dir }
    }
}

/// xdg not support
/// config dir, data dir, cache dir, runtime dir and state dir are
/// all the ~/.gtk-qq
#[allow(dead_code)]
#[cfg(not(any(
    all(target_os = "linux", any(target_arch = "x86_64", target_arch = "i864")),
    all(target_os = "macos", target_arch = "x86_64",)
)))]
impl ResourceDirectories {
    pub fn create_base_dir(&self) -> io::Result<()> {
        if !self.root_dir.exists() {
            logger!(info "create gtk qq dir");
            std::fs::create_dir_all(&self.root_dir)?;
        }

        Ok(())
    }

    pub fn get_config_path(&self) -> PathBuf {
        self.root_dir.join("config.toml")
    }

    pub fn place_config_path(&self) -> io::Result<PathBuf> {
        let res_root = &self.root_dir;
        if !res_root.exists() {
            logger!(info "config location directory need create");
            std::fs::create_dir_all(&res_root)?;
        }
        Ok(res_root.join("config.toml"))
    }

    pub fn get_cache_home(&self) -> PathBuf {
        self.root_dir.clone()
    }
    pub fn get_state_home(&self) -> PathBuf {
        self.root_dir.clone()
    }
    pub fn get_runtime_home(&self) -> PathBuf {
        self.root_dir.clone()
    }
    pub fn get_data_home(&self) -> PathBuf {
        self.root_dir.clone()
    }
}
#[allow(dead_code)]
#[cfg(any(
    all(target_os = "linux", any(target_arch = "x86_64", target_arch = "i864")),
    all(target_os = "macos", target_arch = "x86_64",)
))]
impl ResourceDirectories {
    pub fn create_base_dir(&self) -> io::Result<()> {
        unimplemented!()
    }

    pub fn get_config_path(&self) -> PathBuf {
        unimplemented!()
    }

    pub fn place_config_path(&self) -> io::Result<PathBuf> {
        unimplemented!()
    }

    pub fn get_cache_home(&self) -> PathBuf {
        unimplemented!()
    }
    pub fn get_state_home(&self) -> PathBuf {
        unimplemented!()
    }
    pub fn get_runtime_home(&self) -> PathBuf {
        unimplemented!()
    }
    pub fn get_data_home(&self) -> PathBuf {
        unimplemented!()
    }
}
