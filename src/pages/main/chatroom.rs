use relm4::factory::{positions::StackPageInfo, FactoryPrototype, FactoryVec};
use relm4::gtk::ScrolledWindow;
use relm4::{adw, gtk, Sender};

use adw::prelude::*;
use adw::Avatar;
use gtk::{Box, Label, Orientation, Stack};

use super::MainMsg;

pub struct Chatroom {
    pub username: String,
    pub messages: Vec<String>,
}

#[derive(Debug)]
pub struct ChatroomWidgets {
    root: ScrolledWindow,
}

fn generate_message_widget(username: &String, message: &String) -> Box {
    let root_box = Box::new(Orientation::Horizontal, 2);
    let avatar = Avatar::new(32, Some(username.as_str()), true);
    let right_box = Box::new(Orientation::Vertical, 2);
    let username = Label::new(Some(username.as_str()));
    let message_box = Box::default();
    let message = Label::new(Some(message.as_str()));
    message_box.append(&message);
    right_box.append(&username);
    right_box.append(&message_box);
    root_box.append(&avatar);
    root_box.append(&right_box);
    root_box
}

impl FactoryPrototype for Chatroom {
    type Factory = FactoryVec<Self>;
    type Widgets = ChatroomWidgets;
    type Msg = MainMsg;
    type View = Stack;
    type Root = ScrolledWindow;

    fn init_view(&self, _key: &usize, _sender: Sender<MainMsg>) -> ChatroomWidgets {
        let list = Box::new(Orientation::Vertical, 2);
        for (i, message) in self.messages.iter().enumerate() {
            if i % 2 == 0 {
                list.append(&generate_message_widget(&self.username, message));
            } else {
                list.append(&generate_message_widget(&"You".to_string(), message));
            }
        }
        let window = ScrolledWindow::new();
        window.set_child(Some(&list));

        ChatroomWidgets { root: window }
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
