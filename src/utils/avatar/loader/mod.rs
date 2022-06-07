use std::{future::Future, io, path::PathBuf};

use crate::utils::DirAction;

use super::error::AvatarError;

pub mod user;
pub trait AvatarLoad {
    type Fut: Future<Output = Result<(), AvatarError>>;

    fn download_avatar(id: i64) -> Self::Fut;
    fn get_avatar_location(id: i64,action:DirAction) -> io::Result<PathBuf>;

    fn get_avatar(id:i64)->io::Result<PathBuf>{
        <Self as AvatarLoad>::get_avatar_location(id, DirAction::None)
    }
}