use std::{borrow::Cow, io, path::PathBuf};

use super::AvatarLoader;
use crate::utils::{avatar::open_avatar_dir, DirAction};

pub struct User;

impl AvatarLoader for User {
    fn get_avatar_location_dir(action: DirAction) -> io::Result<PathBuf> {
        open_avatar_dir("users", action)
    }

    fn avatar_download_url(id: i64) -> Cow<'static, String> {
        Cow::Owned(format!(
            "http://q2.qlogo.cn/headimg_dl?dst_uin={}&spec=160",
            &id
        ))
    }
}
