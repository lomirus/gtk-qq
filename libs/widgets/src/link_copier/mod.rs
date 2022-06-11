mod component;
mod payloads;
mod widgets;

pub use component::LinkCopierModel;
pub use payloads::{Input, Output, Payload, State};
pub use widgets::LinkCopierWidgets;

#[allow(dead_code)]
pub type LinkCopier = relm4::Controller<LinkCopierModel>;
