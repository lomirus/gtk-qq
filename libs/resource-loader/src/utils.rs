use std::path::PathBuf;

use tap::Tap;

macro_rules! default_string {
    ($name:ident=> $default:literal) => {
        struct $name;

        impl $name {
            fn get_default() -> String {
                String::from($default)
            }
        }
    };

    {
        $($name:ident=> $default:literal)*
    }=>{
        $(
            default_string!($name => $default);
        )*
    }
}

#[macro_export(crate)]
macro_rules! logger {
    (info $l:literal $(, $v:expr)*) => {
           { 
            #[cfg(feature = "logger")]
            log::info!($l, $($v),*);
            #[cfg(not(feature = "logger"))]
            println!("[INFO] {}",format!($l, $($v),*));
        }
    };
    (debug $l:literal $(, $v:expr)*) => {
        {
            #[cfg(feature = "logger")]
            log::debug!($l , $($v),*);
            #[cfg(not(feature = "logger"))]
            println!("[DEBUG] {}",format!($l, $($v),*));
        }
    };
    (warn $l:literal $(, $v:expr)*) => {
        {
            #[cfg(feature = "logger")]
            log::warn!($l, $($v),*);
            #[cfg(not(feature = "logger"))]
            println!("[WARN] {}",format!($l, $($v),*));
        }
    };
    (trace $l:literal $(, $v:expr)*) => {
        {
            #[cfg(feature = "logger")]
            log::trace!($l, $($v),*);
            #[cfg(not(feature = "logger"))]
            println!("[TRACE] {}",format!($l, $($v),*));
        }
    };
    (error $l:literal $(, $v:expr)*) => {
        {
            #[cfg(feature = "logger")]
            log::error!($l, $($v),*);
            #[cfg(not(feature = "logger"))]
            eprintln!("[ERROR] {}",format!($l, $($v),*));
        }
    };
}


pub(crate) fn resource_root() -> PathBuf {
    dirs::home_dir()
        .expect("User Home directory not exist")
        .join(".gtk-qq")
        .tap(|path|logger!(info "config local directory : {:?}", path))
}
