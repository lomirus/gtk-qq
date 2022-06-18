use std::path::PathBuf;

use derivative::Derivative;
use serde::{Deserialize, Serialize};

use super::{
    avatar::AvatarConfig,
    local_db::{DbConfig, InnerDbConfig},
    InnerAvatarConfig,
};

#[derive(Debug, Serialize, Deserialize, Derivative)]
#[derivative(Default)]
pub struct Config {
    #[derivative(Default(value = "resource_root()"))]
    #[serde(default = "resource_root")]
    root: PathBuf,
    avatar: AvatarConfig,
    database: DbConfig,
}

pub(crate) struct InnerConfig {
    pub(crate) avatar: InnerAvatarConfig,
    pub(crate) database: InnerDbConfig,
}

impl Config {
    pub(crate) fn into_inner(self) -> InnerConfig {
        let root = self.root;

        InnerConfig {
            avatar: self.avatar.into_inner(root.as_path()),
            database: self.database.into_inner(root.as_path()),
        }
    }
}

fn resource_root() -> PathBuf {
    dirs::home_dir()
        .expect("User Home directory not exist")
        .join(".gtk-qq")
}
