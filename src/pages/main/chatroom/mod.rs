mod message;

use std::collections::VecDeque;

use relm4::factory::{DynamicIndex, FactoryComponent, FactoryVecDeque};
use relm4::{adw, gtk, Sender};

use adw::prelude::*;
use gtk::{Box, Orientation, ScrolledWindow, Stack, StackPage};

use super::MainMsg;
pub use message::Message;

#[derive(Debug)]
pub struct Chatroom {
    pub username: String,
    pub messages: FactoryVecDeque<Box, Message, MainMsg>,
}

pub struct ChatroomInitParams {
    pub username: String,
    pub messages: VecDeque<Message>,
}

impl FactoryComponent<Stack, MainMsg> for Chatroom {
    type Widgets = ();
    type Input = MainMsg;
    type Root = ScrolledWindow;
    type Command = ();
    type CommandOutput = ();
    type Output = ();
    type InitParams = ChatroomInitParams;

    fn init_root(&self) -> Self::Root {
        let root = ScrolledWindow::new();
        root.set_child(Some(self.messages.widget()));
        root
    }

    fn init_widgets(
        &mut self,
        index: &DynamicIndex,
        _root: &Self::Root,
        returned_widget: &StackPage,
        _input: &Sender<Self::Input>,
        _output: &Sender<Self::Output>,
    ) -> Self::Widgets {
        let index = index.current_index().to_string();
        let index = index.as_str();
        returned_widget.set_name(index);
        returned_widget.set_title(index);

        ()
    }

    fn init_model(
        init_params: Self::InitParams,
        _index: &DynamicIndex,
        input: &Sender<Self::Input>,
        _output: &Sender<Self::Output>,
    ) -> Self {
        let ChatroomInitParams {
            username,
            messages: messages_src,
        } = init_params;
        let messages_box = Box::new(Orientation::Vertical, 2);
        messages_box.set_css_classes(&["chatroom-box"]);

        let mut messages: FactoryVecDeque<Box, Message, MainMsg> =
            FactoryVecDeque::new(messages_box, input);
        for msg_src in messages_src.iter() {
            messages.push_back(msg_src.clone());
        }
        messages.render_changes();
        Chatroom { username, messages }
    }
}
