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
    pub fn with_set_path(self, path: Option<impl AsRef<Path>>) -> Self {
        Self {
            root_dir: path
                .map(|p| p.as_ref().to_path_buf())
                .unwrap_or(self.root_dir),
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

#[cfg(any(
    all(target_os = "linux", any(target_arch = "x86_64", target_arch = "i864")),
    all(target_os = "macos", target_arch = "x86_64",)
))]
impl ResourceDirectories {
    /// ignore config set directory
    pub fn with_set_path(self, _: impl AsRef<Path>) -> Self {
        self
    }

    pub fn new() -> Self {
        Self {
            root_dir: xdg::BaseDirectories::with_prefix("gtk-qq")
                .expect("XDG Path loading failure")
                .tap(|path| logger!(info "config local directory : {:?}", path.get_config_home())),
        }
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
    pub fn get_config_path(&self) -> PathBuf {
        self.root_dir.get_config_file("config.toml")
    }

    pub fn place_config_path(&self) -> io::Result<PathBuf> {
        self.root_dir.place_config_file("config.toml")
    }

    pub fn get_cache_home(&self) -> PathBuf {
        self.root_dir.get_cache_home()
    }
    pub fn get_state_home(&self) -> PathBuf {
        self.root_dir.get_state_home()
    }
    pub fn get_runtime_home(&self) -> PathBuf {
        self.root_dir.get_runtime_home()
    }
    pub fn get_data_home(&self) -> PathBuf {
        self.root_dir.get_data_home()
    }
}
