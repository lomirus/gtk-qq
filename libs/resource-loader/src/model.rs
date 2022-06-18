use std::path::Path;

use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
struct AvatarConfig {
    base_dir: Option<String>,
    group: Option<String>,
    user: Option<String>,
}
#[derive(Debug, Default, Serialize, Deserialize)]
struct DbConfig {
    local: Option<String>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Config {
    avatar: AvatarConfig,
    database: DbConfig,
}

pub struct InnerAvatarConfig {
    pub group: &'static Path,
    pub user: &'static Path,
}

pub struct InnerDbConfig {
    pub local: &'static Path,
}

pub struct InnerConfig {
    pub avatar: InnerAvatarConfig,
    pub local: InnerDbConfig,
}

impl Drop for InnerAvatarConfig {
    fn drop(&mut self) {
        free_path_ref(self.group);
        free_path_ref(self.user);
    }
}

impl Drop for InnerDbConfig {
    fn drop(&mut self) {
        free_path_ref(self.local)
    }
}

fn free_path_ref(path: &'static Path) {
    let box_path = unsafe { Box::from_raw(path as *const _ as *mut Path) };
    drop(box_path)
}
