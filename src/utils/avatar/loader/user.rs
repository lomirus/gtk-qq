use std::{borrow::Cow, io, path::Path};

use resource_loader::{AvatarUser, SyncCreatePath};

use super::AvatarLoader;
use crate::utils::DirAction;

pub struct User;

impl AvatarLoader for User {
    fn get_avatar_location_dir(action: DirAction) -> io::Result<&'static Path> {
        AvatarUser::do_action_and_get_path(action)
    }

    fn avatar_download_url(id: i64) -> Cow<'static, String> {
        Cow::Owned(format!(
            "http://q2.qlogo.cn/headimg_dl?dst_uin={}&spec=160",
            &id
        ))
    }
}
