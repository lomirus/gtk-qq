mod group;
mod user;

use std::{borrow::Cow, future::Future, io, path::{PathBuf, Path}, pin::Pin};

use super::error::AvatarError;
use crate::utils::DirAction;
pub use group::Group;
use relm4::gtk::{gdk_pixbuf::Pixbuf};
pub use user::User;

pub trait AvatarLoader {
    fn get_avatar_location_dir(action: DirAction) -> io::Result<&'static Path>;
    fn avatar_download_url(id: i64) -> Cow<'static, String>;

    fn download_avatar(
        id: i64,
    ) -> Pin<Box<dyn Future<Output = Result<(), AvatarError>> + Send + Sync>> {
        use tokio::fs::write;

        let filename = <Self as AvatarLoader>::get_avatar_filename(id, DirAction::CreateAll);
        let url = <Self as AvatarLoader>::avatar_download_url(id);

        Box::pin(async move {
            println!("Downloading {}", url);
            let body = reqwest::get(&*url).await?.bytes().await?;

            write(filename?, &body).await?;

            Ok(())
        })
    }

    fn get_avatar_filename(id: i64, action: DirAction) -> io::Result<PathBuf> {
        <Self as AvatarLoader>::get_avatar_location_dir(action)
            .map(|p| p.join(format!("{}.png", id)))
    }

    fn get_avatar(id: i64) -> io::Result<PathBuf> {
        <Self as AvatarLoader>::get_avatar_filename(id, DirAction::None)
    }

    fn get_avatar_as_pixbuf(id: i64, width: i32, height: i32) -> Result<Pixbuf, AvatarError> {
        let path = <Self as AvatarLoader>::get_avatar(id)?;
        if !path.exists() {
            Err(io::Error::new(
                io::ErrorKind::NotFound,
                "Target Avatar Not Found",
            ))?;
        }
        let pix_buf = Pixbuf::from_file_at_size(path, width, height)?;

        Ok(pix_buf)
    }
}
