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
