use relm4::factory::{DynamicIndex, FactoryComponent};
use relm4::{adw, gtk, Sender};

use adw::{prelude::*, Avatar};
use gtk::{Align, Box, Label, ListBox, Orientation, Widget};

use crate::handler::ACCOUNT;

use super::ChatroomMsg;

#[derive(Debug, Clone)]
pub struct MessageGroup {
    pub account: i64,
    pub messages: Vec<String>,
}

impl FactoryComponent<Box, ChatroomMsg> for MessageGroup {
    type Widgets = ();
    type Input = ChatroomMsg;
    type Root = Box;
    type Command = ();
    type CommandOutput = ();
    type InitParams = MessageGroup;
    type Output = ();

    fn init_model(
        message: Self::InitParams,
        _index: &DynamicIndex,
        _input: &Sender<Self::Input>,
        _output: &Sender<Self::Output>,
    ) -> MessageGroup {
        message
    }

    fn init_root(&self) -> Self::Root {
        let root_box = Box::builder()
            .orientation(Orientation::Horizontal)
            .spacing(8)
            .margin_bottom(8)
            .build();

        if &self.account == ACCOUNT.get().unwrap() {
            root_box.set_halign(Align::End)
        }

        root_box
    }

    fn init_widgets(
        &mut self,
        index: &DynamicIndex,
        root: &Self::Root,
        _returned_widget: &Widget,
        input: &Sender<Self::Input>,
        output: &Sender<Self::Output>,
    ) -> Self::Widgets {
        relm4::view! {
            avatar_box = Box {
                set_orientation: Orientation::Vertical,
                Avatar {
                    set_size: 32,
                    set_text: Some(self.account.to_string().as_str()),
                    set_show_initials: true
                }
            }
        }

        relm4::view! {
            main_box = Box {
                set_orientation: Orientation::Vertical,
                set_spacing: 4,
                append: username_box = &Box {
                    Label {
                        set_label: self.account.to_string().as_str(),
                        set_css_classes: &["caption"]
                    }
                },
                append: messages_box = &ListBox {
                    set_css_classes: &["boxed-list"]
                }
            }
        }

        for content in self.messages.iter() {
            relm4::view! {
                message_box = Box {
                    set_css_classes: &["header", "message-box"],
                    Label {
                        set_label: content.as_str(),
                        set_selectable: true
                    }
                }
            }
            messages_box.append(&message_box);
        }

        if &self.account == ACCOUNT.get().unwrap() {
            username_box.set_halign(Align::End);
            root.append(&main_box);
            root.append(&avatar_box);
        } else {
            username_box.set_halign(Align::Start);
            root.append(&avatar_box);
            root.append(&main_box);
        }
    }
}
