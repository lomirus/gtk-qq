mod message;

use relm4::adw::prelude::WidgetExt;
use relm4::factory::{FactoryComponent, FactoryVecDeque, DynamicIndex};
use relm4::{gtk, Sender};

use gtk::{Box, Orientation, ScrolledWindow, Stack,StackPage};

use super::MainMsg;
pub use message::Message;

#[derive(Debug)]
pub struct Chatroom {
    pub username: String,
    pub messages: FactoryVecDeque<Box, Message, MainMsg>,
}

#[derive(Debug)]
pub struct ChatroomWidgets {
    
}

impl FactoryComponent<Stack, MainMsg> for Chatroom {
    type Widgets = ChatroomWidgets;
    type Input = MainMsg;
    type Root = ScrolledWindow;
    type Command = ();
    type CommandOutput = ();
    type Output = ();
    type InitParams = Chatroom;

    fn init_root() -> Self::Root {
        let root = ScrolledWindow::new();
        root
    }

    fn init_widgets(
        &mut self,
        index: &DynamicIndex,
        root: &Self::Root,
        _returned_widget: &StackPage,
        input: &Sender<Self::Input>,
        output: &Sender<Self::Output>,
    ) -> Self::Widgets {
        let list = Box::new(Orientation::Vertical, 2);
        list.set_css_classes(&["chatroom-box"]);
        // self.messages.generate(&list, sender);

        root.set_child(Some(&list));

        ChatroomWidgets { }
    }

    fn init_model(
        init_params: Self::InitParams,
        _index: &DynamicIndex,
        _input: &Sender<Self::Input>,
        _output: &Sender<Self::Output>,
    ) -> Self {
        init_params
    }

    // fn position(&self, index: &usize) -> StackPageInfo {
    //     StackPageInfo {
    //         name: Some(index.to_string()),
    //         title: Some(index.to_string()),
    //     }
    // }
}
