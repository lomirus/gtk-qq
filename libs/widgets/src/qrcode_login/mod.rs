mod background;
mod component;
mod payloads;
mod widgets;

pub use component::QrCodeLoginModel;
pub use payloads::{Input, Output, PayLoad};
pub use widgets::QrCodeLoginWidgets;

pub type QrCodeLogin = relm4::Controller<QrCodeLoginModel>;

