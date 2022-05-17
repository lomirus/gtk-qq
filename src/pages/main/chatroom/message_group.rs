use relm4::factory::{DynamicIndex, FactoryComponent};
use relm4::{adw, gtk, Sender};

use adw::{prelude::*, Avatar};
use gtk::{Align, Box, Label, Orientation, Widget};

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
        let left_box = Box::builder().orientation(Orientation::Vertical).build();
        let right_box = Box::builder()
            .orientation(Orientation::Vertical)
            .spacing(4)
            .build();
        let username_box = Box::builder().build();
        let username = Label::builder()
            .label(self.author.as_str())
            .css_classes(vec!["caption".to_string()])
            .build();
        let messages_box = Box::builder()
            .orientation(Orientation::Vertical)
            .spacing(4)
            .build();
        for content in self.messages.iter() {
            let message_box = Box::builder()
                .css_classes(vec!["message-box".to_string()])
                .halign(Align::Start)
                .build();
            let message = Label::builder()
                .label(content.as_str())
                .selectable(true)
                .build();
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
