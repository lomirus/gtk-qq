use std::{future::Future, io, pin::Pin};

use tokio::io::AsyncWriteExt;

use crate::utils::{
    avatar::{error::AvatarError, open_avatar_dir},
    DirAction,
};

use super::AvatarLoad;

pub struct User;

impl AvatarLoad for User {
    type Fut = Pin<Box<dyn Future<Output = Result<(), AvatarError>>>>;

    fn download_avatar(id: i64) -> Self::Fut {
        Box::pin(async move {
            let filename = <Self as AvatarLoad>::get_avatar_location(id, DirAction::CreateAll)?;

            let mut file = tokio::fs::File::open(filename).await?;

            let body = reqwest::get(user_avatar_url(id)).await?.bytes().await?;

            file.write_all(&body).await?;

            Ok(())
        })
    }

    fn get_avatar_location(id: i64, action: DirAction) -> io::Result<std::path::PathBuf> {
        let path = open_avatar_dir("users", DirAction::None)?;

        if let DirAction::CreateAll = action {
            std::fs::create_dir_all(&path)?;
        }
        Ok(path.join(format!("{}.png", id)))
    }
}

fn user_avatar_url(uid: i64) -> String {
    format!("http://q2.qlogo.cn/headimg_dl?dst_uin={}&spec=160", &uid)
}
