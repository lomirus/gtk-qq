use std::path::Path;

use tap::Tap;

use crate::{logger, static_data::load_cfg};

use super::GetPath;

pub struct TempDir;

impl GetPath for TempDir {
    fn get_path() -> &'static Path {
        load_cfg()
            .tap(|_| logger!(info "loading `Temporary Directory` path"))
            .temporary
            .temp_dir
            .path()
    }

    fn path_for_create() -> Option<&'static Path> {
        None
    }
}

pub struct CaptchaQrCode;

impl GetPath for CaptchaQrCode {
    fn get_path() -> &'static Path {
        load_cfg()
            .tap(|_| logger!(info "loading `Captcha QrCode Picture` path"))
            .temporary
            .captcha_file
    }

    fn path_for_create() -> Option<&'static Path> {
        None
    }
}
