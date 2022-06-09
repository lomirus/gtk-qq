use relm4::gtk::{
    builders::{BoxBuilder, ButtonBuilder, LinkButtonBuilder},
    traits::{BoxExt, ButtonExt, WidgetExt},
    Align, Box, Button, LinkButton, Orientation,
};

enum SetStatus {
    UrISet,
    LabelSet,
    Both,
    None,
}

#[derive(Debug, Clone, Copy)]
pub enum LinkCopierState {
    Both,
    LinkOnly,
    BtnOnly,
}

#[derive(Debug, typed_builder::TypedBuilder)]
pub struct LinkerCopierCfg {
    pub(super) url: String,
    #[builder(default, setter(strip_option))]
    pub(super) label: Option<String>,
    #[builder(default=LinkCopierState::Both)]
    pub(super) ty: LinkCopierState,
}
