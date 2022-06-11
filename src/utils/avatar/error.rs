use std::io;

#[derive(Debug)]
pub enum AvatarError {
    Io(io::Error),
    Request(reqwest::Error),
    Glib(relm4::gtk::glib::Error),
}

impl std::fmt::Display for AvatarError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AvatarError::Io(err) => write!(f, "Avatar Io Error : {}", err),
            AvatarError::Request(err) => write!(f, "Avatar Request Error : {}", err),
            AvatarError::Glib(err) => write!(f, "Avatar GLib Error : {}", err),
        }
    }
}

impl From<io::Error> for AvatarError {
    fn from(err: io::Error) -> Self {
        AvatarError::Io(err)
    }
}

impl From<reqwest::Error> for AvatarError {
    fn from(err: reqwest::Error) -> Self {
        AvatarError::Request(err)
    }
}

impl From<relm4::gtk::glib::Error> for AvatarError {
    fn from(err: relm4::gtk::glib::Error) -> Self {
        AvatarError::Glib(err)
    }
}
