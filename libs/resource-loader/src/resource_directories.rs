use std::{
    io,
    path::{Path, PathBuf},
};

use tap::Tap;

use crate::logger;

pub struct ResourceDirectories {
    project_dir: directories::ProjectDirs,
    // moving
    data_move: Option<PathBuf>,
}

impl ResourceDirectories {
    pub fn with_set_path(mut self, path: Option<impl AsRef<Path>>) -> Self {
        self.data_move = path.map(|path|path.as_ref().to_path_buf());
        self
    }

    pub fn new() -> Self {
        let project_dir = directories::ProjectDirs::from("ricq", "gtk-qq", "gtk-qq")
            .expect("User Home directory not exist")
            .tap(|path| logger!(info "config local directory : {:?}", path.config_dir()));

        Self {
            project_dir,
            data_move: None,
        }
    }
}
#[allow(dead_code)]
impl ResourceDirectories {
    pub fn get_config_path(&self) -> PathBuf {
        self.project_dir.config_dir().join("config.toml")
    }

    pub fn place_config_path(&self) -> io::Result<PathBuf> {
        let config_dir = self.project_dir.config_dir();
        if !config_dir.exists() {
            logger!(info "config location directory need create");
            std::fs::create_dir_all(&config_dir)?;
        }
        Ok(self.get_config_path())
    }

    pub fn get_cache_home(&self) -> PathBuf {
        match self.data_move {
            Some(ref path) => path.join("cache"),
            None => self.project_dir.cache_dir().to_path_buf(),
        }
    }
    pub fn get_state_home(&self) -> Option<PathBuf> {
        match self.data_move {
            Some(ref path) => Some(path.join("state")),
            None => self.project_dir.state_dir().map(Path::to_path_buf),
        }
    }
    pub fn get_data_home(&self) -> PathBuf {
        match self.data_move {
            Some(ref path) => path.join("data"),
            None => self.project_dir.data_dir().to_path_buf(),
        }
    }
    pub fn get_data_local_home(&self) -> PathBuf {
        match self.data_move {
            Some(ref path) => path.join("data_local"),
            None => self.project_dir.data_local_dir().to_path_buf(),
        }
    }
}
