use crate::static_data::load_cfg;

use super::GetPath;

pub struct SqlDataBase;

impl GetPath for SqlDataBase {
    fn get_path() -> &'static std::path::Path {
        let cfg = load_cfg();

        cfg.database.sql_data
    }
}
