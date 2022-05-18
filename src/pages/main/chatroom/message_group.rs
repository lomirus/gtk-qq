use relm4::factory::{DynamicIndex, FactoryComponent};
use relm4::{adw, gtk, Sender};

use adw::{prelude::*, Avatar};
use gtk::{Box, Label, ListBox, Orientation, Widget};

use super::super::MainMsg;

#[derive(Debug, Clone)]
pub struct MessageGroup {
    pub author: String,
    pub messages: Vec<String>,
}

impl FactoryComponent<Box, MainMsg> for MessageGroup {
    type Widgets = ();
    type Input = MainMsg;
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
        let avatar = Avatar::new(32, Some(self.author.as_str()), true);
        let left_box = Box::new(Orientation::Vertical, 0);
        let right_box = Box::new(Orientation::Vertical, 4);
        let username_box = Box::default();

        let username = Label::new(Some(self.author.as_str()));
        username.add_css_class("caption");

        let messages_box = ListBox::new();
        messages_box.add_css_class("boxed-list");

        for content in self.messages.iter() {
            let message_box = Box::default();
            message_box.add_css_class("header");
            message_box.add_css_class("message-box");

            let message = Label::new(Some(content.as_str()));
            message.set_selectable(true);

            message_box.append(&message);
            messages_box.append(&message_box);
        }

        username_box.append(&username);
        left_box.append(&avatar);
        right_box.append(&username_box);
        right_box.append(&messages_box);
        root.append(&left_box);
        root.append(&right_box);

        ()
    }
}
