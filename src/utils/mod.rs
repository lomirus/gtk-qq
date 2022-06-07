use std::{
    io::{self},
    path::PathBuf,
};

use once_cell::sync::OnceCell;

pub mod avatar;

static BASE_DIR: &str = ".gtk-qq";

static BASE_DIR_PATH: OnceCell<PathBuf> = OnceCell::new();

pub(crate) fn init_gtk_qq_dir() -> Result<&'static PathBuf, io::Error> {
    BASE_DIR_PATH.get_or_try_init(|| {
        let path = dirs::home_dir()
            .ok_or(io::Error::new(
                io::ErrorKind::NotFound,
                "user home directory not found",
            ))?
            .join(BASE_DIR);

        std::fs::create_dir(&path)?;

        Ok(path)
    })
}

#[derive(Debug, Clone, Copy)]
pub enum DirAction {
    CreateAll,
    None,
}