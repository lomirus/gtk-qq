use relm4::adw;
use typed_builder::TypedBuilder;

pub enum Output {
    ConfirmVerify,
    CopyLink,
}

#[derive(Debug, TypedBuilder)]
pub struct Payload {
    pub(super) window: adw::Window,
    pub(super) unlock_url: String,
    #[builder(default)]
    pub(super) sms_phone: Option<String>,
}
