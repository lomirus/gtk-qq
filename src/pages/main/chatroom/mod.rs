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
    messages: FactoryVecDeque<Box, MessageGroup, ChatroomMsg>,
    input_box: Box,
}

#[derive(Debug)]
pub enum ChatroomMsg {
    AddMessage(Message),
    SendMessage(Message),
}

pub struct ChatroomInitParams {
    pub username: String,
    pub messages: VecDeque<Message>,
}

impl FactoryComponent<Stack, MainMsg> for Chatroom {
    type Widgets = ();
    type Input = ChatroomMsg;
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

        root.append(&view);
        root.append(&self.input_box);
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

        let messages: FactoryVecDeque<Box, MessageGroup, ChatroomMsg> =
            FactoryVecDeque::new(messages_box, input);
        for msg_src in messages_src.iter() {
            input.send(ChatroomMsg::AddMessage(msg_src.clone()))
        }

        relm4::view! {
            input_box = &Box {
                set_margin_all: 8,
                append: entry = &Entry {
                    set_hexpand: true,
                    set_show_emoji_icon: true,
                    set_placeholder_text: Some("Send a message..."),
                    set_margin_end: 8,
                },
                append = &Button {
                    set_icon_name: "send-symbolic",
                    connect_clicked(input) => move |_| {
                        input.send(ChatroomMsg::SendMessage(Message {
                            author: "You".to_string(),
                            message: entry.buffer().text()
                        }));
                        entry.buffer().set_text("");
                    }
                },
            }
        }

        Chatroom {
            username,
            messages,
            input_box,
        }
    }

    fn update(
        &mut self,
        relm_msg: Self::Input,
        input: &Sender<Self::Input>,
        _output: &Sender<Self::Output>,
    ) -> Option<Self::Command> {
        match relm_msg {
            ChatroomMsg::AddMessage(message) => {
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
                    });
                }

                self.messages.render_changes();
            }
            ChatroomMsg::SendMessage(message) => {
                input.send(ChatroomMsg::AddMessage(message));
            }
        }
        None
    }
}
