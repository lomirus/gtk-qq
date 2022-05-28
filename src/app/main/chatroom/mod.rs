mod message_group;

use std::collections::VecDeque;

use relm4::factory::{DynamicIndex, FactoryComponent, FactoryVecDeque};
use relm4::{adw, gtk, Sender, WidgetPlus};

use adw::prelude::*;
use gtk::{Box, Button, Entry, Orientation, ScrolledWindow, Stack, StackPage};
use ricq::msg::{elem, MessageChain};
use tokio::task;

use crate::handler::CLIENT;

use super::{MainMsg, Message};
use message_group::MessageGroup;

#[derive(Debug)]
pub struct Chatroom {
    pub account: i64,
    pub messages: FactoryVecDeque<Box, MessageGroup, ChatroomMsg>,
    input_box: Box,
}

impl Chatroom {
    pub fn push_message(&mut self, message: Message) {
        if self.messages.is_empty() {
            self.messages.push_back(MessageGroup {
                account: message.sender,
                messages: vec![message.content],
            });
        } else {
            let mut last_message_group = self.messages.pop_back().unwrap();
            if last_message_group.account == message.sender {
                last_message_group.messages.push(message.content);
                self.messages.push_back(last_message_group);
            } else {
                self.messages.push_back(last_message_group);
                self.messages.push_back(MessageGroup {
                    account: message.sender,
                    messages: vec![message.content],
                });
            }
        }

        self.messages.render_changes();
    }
}

async fn send_message(content: String, target: i64, output: Sender<MainMsg>) {
    let client = CLIENT.get().unwrap();
    let message = MessageChain::new(elem::Text::new(content.clone()));
    let res = client.send_friend_message(target, message).await;
    match res {
        Ok(_) => {
            output.send(MainMsg::SendMessage(target, content));
        }
        Err(err) => {
            panic!("err: {:?}", err);
        }
    }
}

#[derive(Debug)]
pub enum ChatroomMsg {
    AddMessage(Message),
    SendMessage(i64, String),
}

pub struct ChatroomInitParams {
    pub account: i64,
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
        _index: &DynamicIndex,
        _root: &Self::Root,
        returned_widget: &StackPage,
        _input: &Sender<Self::Input>,
        _output: &Sender<Self::Output>,
    ) -> Self::Widgets {
        returned_widget.set_name(&self.account.to_string());
        returned_widget.set_title(&self.account.to_string());
    }

    fn init_model(
        init_params: Self::InitParams,
        _index: &DynamicIndex,
        input: &Sender<Self::Input>,
        _output: &Sender<Self::Output>,
    ) -> Self {
        let ChatroomInitParams {
            account,
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
            entry = &Entry {
                set_hexpand: true,
                set_show_emoji_icon: true,
                set_placeholder_text: Some("Send a message..."),
                set_margin_end: 8,
            }
        }

        let entry_buffer = entry.buffer();

        relm4::view! {
            input_box = &Box {
                set_margin_all: 8,
                append: &entry,
                append = &Button {
                    set_icon_name: "send-symbolic",
                    connect_clicked[input] => move |_| {
                        input.send(ChatroomMsg::SendMessage(
                            account,
                            entry_buffer.text()
                        ));
                        entry_buffer.set_text("");
                    }
                },
            }
        }

        Chatroom {
            account,
            messages,
            input_box,
        }
    }

    fn update(
        &mut self,
        relm_msg: Self::Input,
        _input: &Sender<Self::Input>,
        output: &Sender<Self::Output>,
    ) -> Option<Self::Command> {
        match relm_msg {
            ChatroomMsg::AddMessage(message) => self.push_message(message),
            ChatroomMsg::SendMessage(target, content) => {
                task::spawn(send_message(content, target, output.clone()));
            }
        }
        None
    }

    fn output_to_parent_msg(output: Self::Output) -> Option<MainMsg> {
        Some(output)
    }
}
