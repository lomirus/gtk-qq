mod config;
use std::path::Path;

mod avatar;
mod local_db;
mod temp;

fn free_path_ref(path: &'static Path) {
    let box_path = unsafe { Box::from_raw(path as *const _ as *mut Path) };
    #[cfg(test)]{
        println!("droping data {:?}",&box_path);
    }
    drop(box_path)
}

fn static_leak<T: ?Sized>(boxed: Box<T>) -> &'static T {
    Box::leak(boxed) as &'static T
}

pub(crate) use avatar::{AvatarConfig, InnerAvatarConfig};
pub(crate)  use local_db::{DbConfig, InnerDbConfig};
pub(crate)  use config::{Config,InnerConfig};