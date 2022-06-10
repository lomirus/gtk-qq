use std::borrow::Cow;

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]

pub enum State {
    Both,
    LinkOnly,
    BtnOnly,
}

#[allow(dead_code)]
pub enum Input {
    Link(Cow<'static, String>),
    Label(Cow<'static, String>),
    State(State),
}

pub enum Output {
    LinkCopied,
}

#[derive(Debug, typed_builder::TypedBuilder)]
pub struct Payload {
    pub(super) url: String,
    #[builder(default, setter(strip_option))]
    pub(super) label: Option<String>,
    #[builder(default=State::Both)]
    pub(super) ty: State,
}
