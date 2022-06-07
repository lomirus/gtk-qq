pub mod group;
use std::{borrow::Cow, future::Future, io, path::PathBuf, pin::Pin};

use crate::utils::DirAction;

use super::error::AvatarError;

pub mod user;
pub trait AvatarLoad {
    fn get_avatar_location_dir(action: DirAction) -> io::Result<PathBuf>;
    fn avatar_download_url(id: i64) -> Cow<'static, String>;

    fn download_avatar(id: i64) -> Pin<Box<dyn Future<Output = Result<(), AvatarError>>>> {
        use tokio::{fs, io::AsyncWriteExt};

        let filename = <Self as AvatarLoad>::get_avatar_filename(id, DirAction::CreateAll);
        let url = <Self as AvatarLoad>::avatar_download_url(id);

        Box::pin(async move {
            let mut file = fs::File::open(filename?).await?;

            let body = reqwest::get(&*url).await?.bytes().await?;

            file.write_all(&body).await?;

            Ok(())
        })
    }

    fn get_avatar_filename(id: i64, action: DirAction) -> io::Result<PathBuf> {
        <Self as AvatarLoad>::get_avatar_location_dir(action).map(|p| p.join(format!("{}.png", id)))
    }

    fn get_avatar(id: i64) -> io::Result<PathBuf> {
        <Self as AvatarLoad>::get_avatar_filename(id, DirAction::None)
    }
}
