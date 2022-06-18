use std::path::Path;

use crate::static_data::load_cfg;

use super::GetPath;

pub struct User;

impl GetPath for User {
    fn get_path() -> &'static Path {
        let cfg = load_cfg();
        cfg.avatar.user
    }
}

pub struct Group;

impl GetPath for Group {
    fn get_path() -> &'static Path {
        let cfg = load_cfg();
        cfg.avatar.group
    }
}
