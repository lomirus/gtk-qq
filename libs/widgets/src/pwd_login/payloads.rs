use relm4::gtk::gdk::Paintable;

pub enum Input {
    Account(String),
    Password(String),
    Login,
    Avatar(Option<Paintable>),
}

pub enum Output {
    Login { account: i64, pwd: String },
    EnableLogin(bool),
}

#[derive(Debug)]
pub(super) enum State {
    NoChange,
    Update,
}

#[derive(Debug, Default)]
pub struct Payload {
    pub account: Option<i64>,
    pub password: Option<String>,
    pub avatar: Option<Paintable>,
}
