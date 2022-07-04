use std::path::Path;

use crate::{logger, static_data::load_cfg};

use super::GetPath;

pub struct SqlDataBase;

impl GetPath for SqlDataBase {
    fn get_path() -> &'static Path {
        let cfg = load_cfg();
        logger!(info "loading `Sql DataBase` path");
        cfg.database.sql_data
    }

    fn path_for_create() -> Option<&'static Path> {
        <Self as GetPath>::get_path().parent()
    }
}
