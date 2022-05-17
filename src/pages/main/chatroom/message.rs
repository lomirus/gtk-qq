use relm4::factory::{DynamicIndex, FactoryComponent};
use relm4::{adw, gtk, Sender};

use adw::{prelude::*, Avatar};
use gtk::{Align, Box, Label, Orientation};

use super::super::MainMsg;

#[derive(Debug, Clone)]
pub struct Message {
    pub author: String,
    pub content: String,
}

#[derive(Debug)]
pub struct MessageWidgets {}

impl FactoryComponent<Box, MainMsg> for Message {
    type Widgets = MessageWidgets;
    type Input = MainMsg;
    type Root = Box;
    type Command = ();
    type CommandOutput = ();
    type InitParams = Message;
    type Output = ();

    fn init_model(
        message: Self::InitParams,
        _index: &DynamicIndex,
        _input: &Sender<Self::Input>,
        _output: &Sender<Self::Output>,
    ) -> Message {
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
        _returned_widget: &gtk::Widget,
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
        let message_box = Box::builder()
            .css_classes(vec!["message-box".to_string()])
            .halign(Align::Start)
            .build();
        let message = Label::builder()
            .label(self.content.as_str())
            .selectable(true)
            .build();

        message_box.append(&message);
        username_box.append(&username);
        left_box.append(&avatar);
        right_box.append(&username_box);
        right_box.append(&message_box);
        root.append(&left_box);
        root.append(&right_box);
        MessageWidgets {}
    }
}
