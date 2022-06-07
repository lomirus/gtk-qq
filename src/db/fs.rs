use std::path::PathBuf;

use crate::utils::avatar::loader::{AvatarLoader, Group, User};

pub async fn download_user_avatar_file(user_id: i64) {
    User::download_avatar(user_id).await.unwrap();
}

pub fn get_user_avatar_path(user_id: i64) -> PathBuf {
    User::get_avatar(user_id).unwrap()
}

pub async fn download_group_avatar_file(group_id: i64) {
    Group::download_avatar(group_id).await.unwrap();
}

pub fn get_group_avatar_path(group_id: i64) -> PathBuf {
    Group::get_avatar(group_id).unwrap()
}
