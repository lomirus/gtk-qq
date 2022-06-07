pub mod error;
pub mod loader;
use std::{
    io,
    path::{Path, PathBuf},
};

use super::{init_gtk_qq_dir, DirAction};

static AVATARS_DIR: &str = "avatars";

fn init_avatar_dir(action: DirAction) -> io::Result<PathBuf> {
    let base = init_gtk_qq_dir()?.join(AVATARS_DIR);
    if let DirAction::CreateAll = action {
        std::fs::create_dir(&base)?;
    }
    Ok(base)
}

pub(self) fn open_avatar_dir(name: impl AsRef<Path>, action: DirAction) -> io::Result<PathBuf> {
    let path = init_avatar_dir(DirAction::None)?.join(name);
    if let DirAction::CreateAll = action {
        std::fs::create_dir_all(&path)?;
    }
    Ok(path)
}
