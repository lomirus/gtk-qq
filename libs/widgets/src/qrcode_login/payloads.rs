use std::{path::Path, sync::Arc};

use relm4::adw::Window;
use ricq::{Client, LoginResponse, RQError};

pub enum Input {
    UpdateQrCode,
    Updated,
    FollowLogin(LoginResponse),
    Error(RQError),
}

pub enum Output {
    LoginGoAhead(LoginResponse),
    Error(RQError),
}

pub struct PayLoad {
    pub client: Arc<Client>,
    pub windows: Window,
    pub temp_img_path: &'static Path,
}
