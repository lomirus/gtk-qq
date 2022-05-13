use relm4::factory::{positions::StackPageInfo, FactoryPrototype, FactoryVec};
use relm4::{adw, gtk};

use adw::prelude::*;
use gtk::{Box, Label, Stack};

use super::MainMsg;

pub struct Chatroom {
    pub username: String,
    pub messages: Vec<String>,
}

#[relm4::factory_prototype(pub)]
impl FactoryPrototype for Chatroom {
    type Factory = FactoryVec<Self>;
    type Widgets = ChatroomWidgets;
    type Msg = MainMsg;
    type View = Stack;

    view! {
        Box {
            append = &Label {
                set_text: args!(format!("{}: ", self.username).as_str()),
            },
            append = &Label {
                set_text: self.messages.join(", ").as_str(),
            },
        }
    }

    fn position(&self, index: &usize) -> StackPageInfo {
        StackPageInfo {
            name: Some(index.to_string()),
            title: Some(index.to_string()),
        }
    }
}
