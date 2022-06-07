use std::{
    io::{self},
    path::PathBuf,
};

pub mod avatar;

static BASE_DIR: &str = ".gtk-qq";

pub(crate) fn init_gtk_qq_dir() -> Result<PathBuf, io::Error> {
    let mut path = dirs::home_dir().ok_or(io::Error::new(
        io::ErrorKind::NotFound,
        "user home directory not found",
    ))?;
    std::fs::create_dir(&path)?;
    path.push(BASE_DIR);

    Ok(path)
}
