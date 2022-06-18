use std::path::{Path, PathBuf};

use derivative::Derivative;
use serde::{Deserialize, Serialize};

use super::{
    avatar::AvatarConfig, free_path_ref, local_db::DbConfig, static_leak, InnerAvatarConfig,
    InnerDbConfig,
};

#[derive(Debug, Serialize, Deserialize, Derivative)]
#[derivative(Default)]
pub struct Config {
    #[derivative(Default(value = "resource_root()"))]
    #[serde(default = "resource_root")]
    #[serde(alias = "res", alias = "resource")]
    resource_root: PathBuf,
    #[derivative(Default(value = "template_root()"))]
    #[serde(default = "template_root")]
    #[serde(alias = "template", alias = "temp")]
    template_dir: PathBuf,
    avatar: AvatarConfig,
    database: DbConfig,
}

pub struct InnerConfig {
    pub(crate) template_dir: &'static Path,
    pub(crate) avatar: InnerAvatarConfig,
    pub(crate) database: InnerDbConfig,
}

impl Config {
    pub(crate) fn into_inner(self) -> InnerConfig {
        let root = self.resource_root;
        let template_dir = static_leak(self.template_dir.into_boxed_path());

        InnerConfig {
            avatar: self.avatar.into_inner(root.as_path()),
            database: self.database.into_inner(root.as_path()),
            template_dir,
        }
    }
}

impl Drop for InnerConfig {
    fn drop(&mut self) {
        free_path_ref(self.template_dir);
    }
}

fn resource_root() -> PathBuf {
    dirs::home_dir()
        .expect("User Home directory not exist")
        .join(".gtk-qq")
}

fn template_root() -> PathBuf {
    dirs::template_dir()
        .expect("Template Directory Not Exist")
        .join(".gtk-qq")
}
