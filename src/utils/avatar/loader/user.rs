use std::{borrow::Cow, io};

use crate::utils::{avatar::open_avatar_dir, DirAction};

use super::AvatarLoad;

pub struct User;

impl AvatarLoad for User {
    fn get_avatar_location_dir(action: DirAction) -> io::Result<std::path::PathBuf> {
        open_avatar_dir("users", action)
    }

    fn avatar_download_url(id: i64) -> std::borrow::Cow<'static, String> {
        Cow::Owned(user_avatar_url(id))
    }
}

fn user_avatar_url(uid: i64) -> String {
    format!("http://q2.qlogo.cn/headimg_dl?dst_uin={}&spec=160", &uid)
}
