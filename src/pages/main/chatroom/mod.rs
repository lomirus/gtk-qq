mod message;

use relm4::adw::prelude::WidgetExt;
use relm4::factory::Factory;
use relm4::factory::{positions::StackPageInfo, FactoryPrototype, FactoryVec};
use relm4::{gtk, Sender};

use gtk::{Box, Orientation, ScrolledWindow, Stack};

use super::MainMsg;
pub use message::Message;

pub struct Chatroom {
    pub username: String,
    pub messages: FactoryVec<Message>,
}

#[derive(Debug)]
pub struct ChatroomWidgets {
    root: ScrolledWindow,
}

impl FactoryPrototype for Chatroom {
    type Factory = FactoryVec<Self>;
    type Widgets = ChatroomWidgets;
    type Msg = MainMsg;
    type View = Stack;
    type Root = ScrolledWindow;

    fn init_view(&self, _key: &usize, sender: Sender<MainMsg>) -> ChatroomWidgets {
        let list = Box::new(Orientation::Vertical, 2);
        list.set_css_classes(&["chatroom-box"]);
        self.messages.generate(&list, sender);

        let root = ScrolledWindow::new();
        root.set_child(Some(&list));

        ChatroomWidgets { root }
    }

    fn view(&self, _key: &usize, _widgets: &ChatroomWidgets) {}

    fn position(&self, index: &usize) -> StackPageInfo {
        StackPageInfo {
            name: Some(index.to_string()),
            title: Some(index.to_string()),
        }
    }

    fn root_widget(widgets: &ChatroomWidgets) -> &ScrolledWindow {
        &widgets.root
    }
}
