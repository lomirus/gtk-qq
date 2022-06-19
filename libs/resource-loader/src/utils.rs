use std::path::PathBuf;

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

pub(crate) fn resource_root() -> PathBuf {
    dirs::home_dir()
        .expect("User Home directory not exist")
        .join(".gtk-qq")
}
