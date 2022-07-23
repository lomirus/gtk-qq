mod ops;
#[macro_use]
mod utils;
mod configs;
mod static_data;

pub use configs::Config;

pub use static_data::ResourceConfig;

pub use ops::{
    avatar::{Group as AvatarGroup, User as AvatarUser},
    client::{Device, Protocol},
    database::SqlDataBase,
    temporary::{CaptchaQrCode, TempDir},
    AsyncCreatePath, AsyncLoadResource, DirAction, GetPath, SyncCreatePath, SyncLoadResource,
};
