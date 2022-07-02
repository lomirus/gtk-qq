pub mod avatar;
pub mod database;
pub mod temporary;
use std::path::Path;

pub trait GetPath {
    fn get_path() -> &'static Path;

    fn path_for_create() -> Option<&'static Path> {
        <Self as GetPath>::get_path().into()
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum DirAction {
    CreateAll,
    None,
}

pub use sync_ops::{SyncCreatePath, SyncLoadResource};

mod sync_ops {
    use std::{fs::create_dir_all, io, path::Path};

    use tap::Tap;

    use crate::{logger, DirAction, GetPath};

    pub trait SyncCreatePath: GetPath {
        fn create_and_get_path() -> io::Result<&'static Path> {
            if let Some(path) = <Self as GetPath>::path_for_create()
                .tap(|path| logger!(debug "create path {:?}", path))
            {
                create_dir_all(path)?;
            }
            Ok(<Self as GetPath>::get_path().tap(|path| logger!(info "get path: {:?}", path)))
        }

        fn do_action_and_get_path(action: DirAction) -> io::Result<&'static Path> {
            logger!(info "Directory action : {:?}", action);
            match action {
                DirAction::CreateAll => <Self as SyncCreatePath>::create_and_get_path(),
                DirAction::None => Ok(<Self as GetPath>::get_path()),
            }
        }
    }

    impl<T> SyncCreatePath for T where T: GetPath {}

    pub trait SyncLoadResource<Res> {
        type Args;
        type Error: std::error::Error;
        fn load_resource(args: Self::Args) -> Result<Res, Self::Error>;
    }
}

pub use async_ops::{AsyncCreatePath, AsyncLoadResource};

mod async_ops {
    use std::{future::Future, io, path::Path, pin::Pin};

    use tokio::fs::create_dir_all;

    use crate::{logger, DirAction, GetPath};

    pub trait AsyncCreatePath: GetPath {
        fn create_and_get_path_async(
        ) -> Pin<Box<dyn Future<Output = io::Result<&'static Path>> + Send + Sync>> {
            let create_path = <Self as GetPath>::path_for_create();
            logger!(debug "create path {:?}", create_path);
            let path = <Self as GetPath>::get_path();
            logger!(info "get path: {:?}", path);
            Box::pin(async move {
                if let Some(path) = create_path {
                    create_dir_all(path).await?;
                }
                Ok(path)
            })
        }

        fn do_action_and_get_path_async(
            action: DirAction,
        ) -> Pin<Box<dyn Future<Output = io::Result<&'static Path>> + Send + Sync>> {
            logger!(info "Directory action : {:?}", action);
            Box::pin(async move {
                match action {
                    DirAction::CreateAll => {
                        <Self as AsyncCreatePath>::create_and_get_path_async().await
                    }
                    DirAction::None => Ok(<Self as GetPath>::get_path()),
                }
            })
        }
    }

    impl<T> AsyncCreatePath for T where T: GetPath {}

    pub trait AsyncLoadResource<Res> {
        type Fut: Future<Output = Result<Res, Self::Error>> + Send + Sync;
        type Error: std::error::Error;
        type Args;

        fn load_resource_async(args: Self::Args) -> Self::Fut;
    }
}
