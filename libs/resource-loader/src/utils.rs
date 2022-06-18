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
