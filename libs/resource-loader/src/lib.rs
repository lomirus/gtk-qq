mod ops;
#[macro_use]
mod utils;
mod configs;
mod static_data;

pub use configs::Config;

pub use static_data::set_config;

pub use ops::{
    avatar::{Group as AvatarGroup, User as AvatarUser},
    database::SqlDataBase,
    template::Template,
    GetPath,
    DirAction
};
