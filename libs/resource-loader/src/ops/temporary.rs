use std::path::Path;

use crate::static_data::load_cfg;

use super::GetPath;

pub struct TempDir;

impl GetPath for TempDir {
    fn get_path() -> &'static Path {
        load_cfg().temporary.temp_dir.path()
    }

    fn path_for_create() -> Option<&'static Path> {
        None
    }
}

pub struct CaptchaQrCode;

impl GetPath for CaptchaQrCode {
    fn get_path() -> &'static Path {
        load_cfg().temporary.captcha_file
    }

    fn path_for_create() -> Option<&'static Path> {
        None
    }
}
