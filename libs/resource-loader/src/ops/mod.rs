pub mod avatar;
pub mod database;
pub mod template;
use std::{fs::create_dir_all, io, path::Path};

pub trait GetPath {
    fn get_path() -> &'static Path;

    fn create_path() -> Option<&'static Path> {
        <Self as GetPath>::get_path().into()
    }

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
            DirAction::CreateAll => <Self as GetPath>::get_and_create(),
            DirAction::None => Ok(<Self as GetPath>::get_path()),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum DirAction {
    CreateAll,
    None,
}

#[cfg(feature="async-ops")]
pub use async_ops::AsyncCreatePath;

#[cfg(feature = "async-ops")]
mod async_ops {
    use std::{future::Future, io, path::Path, pin::Pin};

    use tokio::fs::create_dir_all;

    use crate::{DirAction, GetPath};

    type PinFuture = Pin<Box<dyn Future<Output = io::Result<&'static Path>> + Send + Sync>>;

    pub trait AsyncCreatePath: GetPath {
        fn get_and_create_async() -> PinFuture;

        fn get_and_do_action_async(action: DirAction) -> PinFuture;
    }

    impl<T> AsyncCreatePath for T
    where
        T: GetPath,
    {
        fn get_and_create_async() -> PinFuture {
            let create_path = <Self as GetPath>::create_path();
            let path = <Self as GetPath>::get_path();
            Box::pin(async move {
                if let Some(path) = create_path {
                    create_dir_all(path).await?;
                }
                Ok(path)
            })
        }

        fn get_and_do_action_async(action: DirAction) -> PinFuture {
            Box::pin(async move {
                match action {
                    DirAction::CreateAll => <Self as AsyncCreatePath>::get_and_create_async().await,
                    DirAction::None => Ok(<Self as GetPath>::get_path()),
                }
            })
        }
    }
}
