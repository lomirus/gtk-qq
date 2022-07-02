mod config;
use std::path::Path;

mod avatar;
mod local_db;
mod temporary;

fn free_path_ref(path: &'static Path) {
    let box_path = unsafe { Box::from_raw(path as *const _ as *mut Path) };
    logger!(trace "dropping Path=> {:?}", &box_path);
    drop(box_path)
}

fn static_leak<T: ?Sized>(boxed: Box<T>) -> &'static T {
    Box::leak(boxed) as &'static T
}

pub(crate) use avatar::InnerAvatarConfig;
pub use config::{Config, InnerConfig};
pub(crate) use local_db::InnerDbConfig;
