use std::path::Path;

use crate::{logger, static_data::load_cfg};

use super::GetPath;

pub struct User;

impl GetPath for User {
    fn get_path() -> &'static Path {
        let cfg = load_cfg();
        logger!(info "loading `User Avatar` path");
        cfg.avatar.user
    }
}

pub struct Group;

impl GetPath for Group {
    fn get_path() -> &'static Path {
        let cfg = load_cfg();
        logger!(info "loading `Group Avatar` path");
        cfg.avatar.group
    }
}
