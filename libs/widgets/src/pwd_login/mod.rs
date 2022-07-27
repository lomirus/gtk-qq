mod component;
mod payloads;
mod widgets;

pub use component::PasswordLoginModel;
pub use payloads::{Input, Output, Payload};

pub use widgets::PwdLoginWidget;

pub type PasswordLogin = relm4::component::Controller<PasswordLoginModel>;
