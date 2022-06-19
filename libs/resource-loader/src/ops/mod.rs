pub mod avatar;
pub mod database;
pub mod temporary;
use std::path::Path;

pub trait GetPath {
    fn get_path() -> &'static Path;

    fn create_path() -> Option<&'static Path> {
        <Self as GetPath>::get_path().into()
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum DirAction {
    CreateAll,
    None,
}

pub use sync_ops::{SyncCreatePath,SyncLoadResource};

mod sync_ops {
    use std::{fs::create_dir_all, io, path::Path};

    use crate::{DirAction, GetPath};

    pub trait SyncCreatePath: GetPath {
        fn get_and_create() -> io::Result<&'static Path> {
            #[cfg(feature = "logger")]
            log::debug!("get and create path {:?}", path);

            if let Some(path) = <Self as GetPath>::create_path() {
                create_dir_all(path)?;
            }
            Ok(<Self as GetPath>::get_path())
        }

        fn get_and_do_action(action: DirAction) -> io::Result<&'static Path> {
            match action {
                DirAction::CreateAll => <Self as SyncCreatePath>::get_and_create(),
                DirAction::None => Ok(<Self as GetPath>::get_path()),
            }
        }
    }

    impl<T> SyncCreatePath for T where T: GetPath {}

    pub trait SyncLoadResource<Res>: GetPath {
        type Args;
        type Error: std::error::Error;
        fn load_resource(args: Self::Args) -> Result<Res, Self::Error>;
    }
}

pub use async_ops::{AsyncCreatePath,AsyncLoadResource};

mod async_ops {
    use std::{future::Future, io, path::Path, pin::Pin};

    use tokio::fs::create_dir_all;

    use crate::{DirAction, GetPath};

    pub trait AsyncCreatePath: GetPath {
        fn get_and_create_async(
        ) -> Pin<Box<dyn Future<Output = io::Result<&'static Path>> + Send + Sync>> {
            let create_path = <Self as GetPath>::create_path();
            let path = <Self as GetPath>::get_path();
            Box::pin(async move {
                if let Some(path) = create_path {
                    create_dir_all(path).await?;
                }
                Ok(path)
            })
        }

        fn get_and_do_action_async(
            action: DirAction,
        ) -> Pin<Box<dyn Future<Output = io::Result<&'static Path>> + Send + Sync>> {
            Box::pin(async move {
                match action {
                    DirAction::CreateAll => <Self as AsyncCreatePath>::get_and_create_async().await,
                    DirAction::None => Ok(<Self as GetPath>::get_path()),
                }
            })
        }
    }

    impl<T> AsyncCreatePath for T where T: GetPath {}

    pub trait AsyncLoadResource<Res>: GetPath {
        type Fut: Future<Output = Result<Res, Self::Error>> + Send + Sync;
        type Error: std::error::Error;
        type Args;

        fn load_resource_async(args: Self::Args) -> Self::Fut;
    }
}
