mod message_group;

use std::collections::VecDeque;

use relm4::factory::{DynamicIndex, FactoryComponent, FactoryVecDeque};
use relm4::{adw, gtk, Sender, WidgetPlus};

use adw::prelude::*;
use gtk::{Box, Button, Entry, Orientation, ScrolledWindow, Stack, StackPage};

use super::{MainMsg, Message};
use message_group::MessageGroup;

#[derive(Debug)]
pub struct Chatroom {
    pub username: String,
    pub messages: FactoryVecDeque<Box, MessageGroup, MainMsg>,
}

pub enum ChatroomRelmMessage {
    AddMessage(Message),
}

pub struct ChatroomInitParams {
    pub username: String,
    pub messages: VecDeque<Message>,
}

impl FactoryComponent<Stack, MainMsg> for Chatroom {
    type Widgets = ();
    type Input = ChatroomRelmMessage;
    type Root = Box;
    type Command = ();
    type CommandOutput = ();
    type Output = MainMsg;
    type InitParams = ChatroomInitParams;

    fn init_root(&self) -> Self::Root {
        let root = Box::new(Orientation::Vertical, 0);

        relm4::view! {
            view = &ScrolledWindow {
                set_vexpand: true,
                set_hexpand: true,
                set_child: Some(self.messages.widget())
            }
        }

        relm4::view! {
            input = &Box {
                set_margin_all: 8,
                append = &Entry {
                    set_hexpand: true,
                    set_show_emoji_icon: true,
                    set_placeholder_text: Some("Send a message..."),
                    set_margin_end: 8
                },
                append = &Button {
                    set_icon_name: "send-symbolic",
                },
            }
        }
        
        root.append(&view);
        root.append(&input);
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
    }

    fn init_model(
        init_params: Self::InitParams,
        _index: &DynamicIndex,
        input: &Sender<Self::Input>,
        output: &Sender<Self::Output>,
    ) -> Self {
        let ChatroomInitParams {
            username,
            messages: messages_src,
        } = init_params;
        let messages_box = Box::new(Orientation::Vertical, 2);
        messages_box.set_css_classes(&["chatroom-box"]);

        let messages: FactoryVecDeque<Box, MessageGroup, MainMsg> =
            FactoryVecDeque::new(messages_box, output);
        for msg_src in messages_src.iter() {
            input.send(ChatroomRelmMessage::AddMessage(msg_src.clone()))
        }
        Chatroom { username, messages }
    }

    fn update(
        &mut self,
        relm_msg: Self::Input,
        _input: &Sender<Self::Input>,
        _output: &Sender<Self::Output>,
    ) -> Option<Self::Command> {
        match relm_msg {
            ChatroomRelmMessage::AddMessage(message) => {
                if !self.messages.is_empty() {
                    let mut last_message_group = self.messages.pop_back().unwrap();
                    if last_message_group.author == message.author {
                        last_message_group.messages.push(message.message);
                        self.messages.push_back(last_message_group);
                    } else {
                        self.messages.push_back(last_message_group);
                        self.messages.push_back(MessageGroup {
                            author: message.author,
                            messages: vec![message.message],
                        });
                    }
                } else {
                    self.messages.push_back(MessageGroup {
                        author: message.author,
                        messages: vec![message.message],
                    })
                }
            }
        }
        self.messages.render_changes();
        None
    }
}
