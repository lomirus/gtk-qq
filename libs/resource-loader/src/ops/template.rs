use crate::static_data::load_cfg;

use super::GetPath;

pub struct Template;

impl GetPath for Template {
    fn get_path() -> &'static std::path::Path {
        load_cfg().template_dir
    }
}
