use std::path::PathBuf;

use derivative::Derivative;
use serde::{Deserialize, Serialize};

use super::{
    avatar::AvatarConfig,
    client::{ClientConfig, ClientInner},
    local_db::DbConfig,
    temporary::{InnerTemporaryConfig, TemporaryConfig},
    InnerAvatarConfig, InnerDbConfig,
};

use crate::resource_directories::ResourceDirectories;

#[derive(Debug, Serialize, Deserialize, Derivative)]
#[derivative(Default)]
pub struct Config {
    #[derivative(Default(value = "None"))]
    #[serde(default = "Default::default")]
    #[serde(alias = "res", alias = "resource")]
    #[serde(skip_serializing_if = "Option::is_none")]
    resource_root: Option<PathBuf>,
    #[serde(default = "Default::default")]
    #[serde(alias = "temp", alias = "temporary")]
    temporary: TemporaryConfig,
    #[serde(default = "Default::default")]
    avatar: AvatarConfig,
    #[serde(default = "Default::default")]
    database: DbConfig,
    #[serde(default = "Default::default")]
    client: ClientConfig,
}

pub struct InnerConfig {
    pub(crate) temporary: InnerTemporaryConfig,
    pub(crate) avatar: InnerAvatarConfig,
    pub(crate) database: InnerDbConfig,
    pub(crate) client: ClientInner,
}

impl Config {
    pub(crate) fn into_inner(self, root: ResourceDirectories) -> InnerConfig {
        let root = root.with_set_path(self.resource_root);
        InnerConfig {
            avatar: self.avatar.into_inner(&root),
            database: self.database.into_inner(&root),
            temporary: self.temporary.into_inner(),
            client: self.client.into(),
        }
    }
}
