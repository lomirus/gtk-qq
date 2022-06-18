use std::{borrow::Cow, io, path::Path};

use resource_loader::{AvatarUser, GetPath};

use super::AvatarLoader;
use crate::utils::DirAction;

pub struct User;

impl AvatarLoader for User {
    fn get_avatar_location_dir(action: DirAction) -> io::Result<&'static Path> {
        AvatarUser::get_and_do_action(action)
    }

    fn avatar_download_url(id: i64) -> Cow<'static, String> {
        Cow::Owned(format!(
            "http://q2.qlogo.cn/headimg_dl?dst_uin={}&spec=160",
            &id
        ))
    }
}
