use std::path::Path;

use serde::{Deserialize, Serialize};

use super::{free_path_ref, static_leak};
use derivative::Derivative;
default_string! {
    BaseDir=>"database"
    SqlData=>"sql_db.db"
}

#[derive(Debug, Serialize, Deserialize, Derivative)]
#[derivative(Default)]
pub struct DbConfig {
    #[derivative(Default(value = "BaseDir::get_default()"))]
    #[serde(default = "BaseDir::get_default")]
    #[serde(alias = "base")]
    base_dir: String,

    #[derivative(Default(value = "SqlData::get_default()"))]
    #[serde(default = "SqlData::get_default")]
    #[serde(alias = "app_db")]
    sql_data: String,
}

pub(crate) struct InnerDbConfig {
    pub sql_data: &'static Path,
}

impl DbConfig {
    pub(crate) fn into_inner(self, base: &Path) -> InnerDbConfig {
        let base = base.join(&self.base_dir);

        let sql_data = static_leak(base.join(&self.sql_data).into_boxed_path());

        InnerDbConfig { sql_data }
    }
}

impl Drop for InnerDbConfig {
    fn drop(&mut self) {
        free_path_ref(self.sql_data)
    }
}
