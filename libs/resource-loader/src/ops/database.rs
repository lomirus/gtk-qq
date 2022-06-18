use std::path::Path;

use crate::static_data::load_cfg;

use super::GetPath;

pub struct SqlDataBase;

impl GetPath for SqlDataBase {
    fn get_path() -> &'static Path {
        let cfg = load_cfg();

        cfg.database.sql_data
    }

    fn create_path() -> Option<&'static Path> {
        <Self as GetPath>::get_path().parent()
    }
}
