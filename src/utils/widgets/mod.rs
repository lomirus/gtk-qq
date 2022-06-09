use relm4::OnDestroy;

pub mod linker_copier;

pub trait CustomWidget {
    type Root: std::fmt::Debug + OnDestroy;
    type InitParams: 'static;
    type Widgets: 'static;
    fn init_root() -> Self::Root;

    fn init(params: Self::InitParams, root: &Self::Root) -> Self::Widgets;
}

#[allow(dead_code)]
pub fn new_widget<C: CustomWidget>(params: C::InitParams) -> C::Root {
    let root = C::init_root();

    C::init(params, &root);

    root
}
