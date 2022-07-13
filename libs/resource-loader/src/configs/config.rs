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

use crate::utils::resource_root;

#[derive(Debug, Serialize, Deserialize, Derivative)]
#[derivative(Default)]
pub struct Config {
    #[derivative(Default(value = "resource_root()"))]
    #[serde(default = "resource_root")]
    #[serde(alias = "res", alias = "resource")]
    resource_root: PathBuf,
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
    pub(crate) fn into_inner(self) -> InnerConfig {
        let root = self.resource_root;

        InnerConfig {
            avatar: self.avatar.into_inner(root.as_path()),
            database: self.database.into_inner(root.as_path()),
            temporary: self.temporary.into_inner(),
            client: self.client.into(),
        }
    }
}
