use relm4::factory::{FactoryPrototype, FactoryVec};
use relm4::{adw, gtk, Sender};

use adw::{prelude::*, Avatar};
use gtk::{Align, Box, Label, Orientation};

use super::super::MainMsg;

pub struct Message {
    pub author: String,
    pub content: String,
}

#[derive(Debug)]
pub struct MessageWidgets {
    root: Box,
}

impl FactoryPrototype for Message {
    type Factory = FactoryVec<Self>;
    type Widgets = MessageWidgets;
    type Msg = MainMsg;
    type View = Box;
    type Root = Box;

    fn init_view(&self, _key: &usize, _sender: Sender<MainMsg>) -> MessageWidgets {
        let root_box = Box::builder()
            .orientation(Orientation::Horizontal)
            .spacing(8)
            .margin_bottom(8)
            .build();
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
        root_box.append(&left_box);
        root_box.append(&right_box);

        MessageWidgets { root: root_box }
    }

    fn view(&self, _key: &usize, _widgets: &MessageWidgets) {}

    fn position(&self, _index: &usize) {}

    fn root_widget(widgets: &MessageWidgets) -> &Box {
        &widgets.root
    }
}
