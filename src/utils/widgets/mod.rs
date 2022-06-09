use relm4::gtk::{prelude::IsA, Widget};

pub mod linker_copier;

/// 摆烂级
pub trait CustomWidget<Message> {
    type Widget: IsA<Widget>;

    fn into_widget(self) -> Self::Widget;
    fn into_widget_ref(&self) -> &Self::Widget;
}

pub trait InternalBuilder<W> {
    fn get_internal(&mut self)->&mut W;   
}