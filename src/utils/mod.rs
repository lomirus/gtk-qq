pub mod avatar;
pub mod widgets;

use std::{io, path::PathBuf};

use once_cell::sync::OnceCell;

const BASE_DIR: &str = ".gtk-qq";

static BASE_DIR_PATH: OnceCell<PathBuf> = OnceCell::new();

pub(crate) fn init_gtk_qq_dir() -> Result<&'static PathBuf, io::Error> {
    BASE_DIR_PATH.get_or_try_init(|| {
        let path = dirs::home_dir()
            .ok_or_else(|| {
                io::Error::new(io::ErrorKind::NotFound, "user home directory not found")
            })?
            .join(BASE_DIR);

        if !path.exists() {
            std::fs::create_dir(&path)?;
        }

        Ok(path)
    })
}

#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
pub enum DirAction {
    CreateAll,
    None,
}
