use std::path::Path;

pub enum Input {
    UpdateQrCode,
}

pub enum Output {}

pub struct PayLoad {
    pub temp_img_path: &'static Path,
}
