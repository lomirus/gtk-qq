use crate::utils::avatar::open_avatar_dir;

use super::AvatarLoad;

pub struct Group;

impl AvatarLoad for Group {
    fn get_avatar_location_dir(
        action: crate::utils::DirAction,
    ) -> std::io::Result<std::path::PathBuf> {
        open_avatar_dir("groups", action)
    }

    fn avatar_download_url(id: i64) -> std::borrow::Cow<'static, String> {
        std::borrow::Cow::Owned(format!("https://p.qlogo.cn/gh/{}/{}/0", id, id))
    }
}
