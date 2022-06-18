use std::{borrow::Cow, io, path::Path};

use resource_loader::{AvatarGroup, SyncCreatePath};

use super::AvatarLoader;
use crate::utils::DirAction;

pub struct Group;

impl AvatarLoader for Group {
    fn get_avatar_location_dir(action: DirAction) -> io::Result<&'static Path> {
        AvatarGroup::get_and_do_action(action)
    }

    fn avatar_download_url(id: i64) -> Cow<'static, String> {
        Cow::Owned(format!("https://p.qlogo.cn/gh/{}/{}/0", id, id))
    }
}
