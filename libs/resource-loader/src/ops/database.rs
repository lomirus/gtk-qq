use std::{io, path::Path, fs::create_dir_all};

use crate::static_data::load_cfg;

use super::GetPath;

pub struct SqlDataBase;

impl GetPath for SqlDataBase {
    fn get_path() -> &'static Path {
        let cfg = load_cfg();

        cfg.database.sql_data
    }

    fn get_and_create_path() -> io::Result<&'static Path> {
        let path = <Self as GetPath>::get_path();

        if let Some(path) = path.parent() {
            create_dir_all(path)?;
        }

        Ok(path)
    }
}
