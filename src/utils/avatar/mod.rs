use std::{
    future::Future,
    io,
    path::{Path, PathBuf},
};

use super::init_gtk_qq_dir;

static AVATARS_DIR: &str = "avatars";

pub trait AvatarLoad {
    type Fut: Future<Output = Result<(), AvatarError>>;

    fn download_avatar(id: i64) -> Self::Fut;
    fn get_avatar(id: i64) -> Option<PathBuf>;
}

pub enum AvatarError {
    Io(io::Error),
    Request(reqwest::Error),
}

fn init_avatar_dir() -> io::Result<PathBuf> {
    let mut base = init_gtk_qq_dir()?;
    base.push(AVATARS_DIR);

    std::fs::create_dir(&base)?;
    Ok(base)
}

pub(self) fn open_avatar_dir(name: impl AsRef<Path>) -> io::Result<PathBuf> {
    let path = init_avatar_dir()?.join(name);
    std::fs::create_dir(&path)?;
    Ok(path)
}
