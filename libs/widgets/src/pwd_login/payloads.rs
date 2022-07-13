use relm4::gtk::gdk::Paintable;

pub enum Input {
    Account(String),
    Password(String),
    Login
}

pub enum Output {
    Login { account: i64, pwd: String },
}

#[derive(Debug, typed_builder::TypedBuilder)]
pub struct Payload {
    pub(super) account: Option<i64>,
    pub(super) password: Option<String>,
    pub(super) avatar : Option<Paintable>
}
