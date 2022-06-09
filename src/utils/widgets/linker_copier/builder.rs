


#[allow(dead_code)]
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
