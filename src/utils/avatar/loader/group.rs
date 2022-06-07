use std::{borrow::Cow, io, path::PathBuf};

use super::AvatarLoader;
use crate::utils::{avatar::open_avatar_dir, DirAction};

pub struct Group;

impl AvatarLoader for Group {
    fn get_avatar_location_dir(action: DirAction) -> io::Result<PathBuf> {
        open_avatar_dir("groups", action)
    }

    fn avatar_download_url(id: i64) -> Cow<'static, String> {
        Cow::Owned(format!("https://p.qlogo.cn/gh/{}/{}/0", id, id))
    }
}
