use relm4::adw;
use typed_builder::TypedBuilder;

pub enum Output {
    ConfirmVerify,
    CopyLink,
}

#[derive(Debug, TypedBuilder)]
pub struct Payload {
    pub(super) unlock_url: String,
    pub(super) window: adw::Window,
}
