use relm4::factory::{DynamicIndex, FactoryComponent};
use relm4::{adw, gtk, Sender, WidgetPlus};

use adw::{prelude::*, Avatar};
use gtk::gdk_pixbuf::Pixbuf;
use gtk::{Align, Box, Label, Orientation, Picture, Widget};
use tokio::task;

use crate::db::fs::{download_user_avatar_file, get_user_avatar_path};
use crate::handler::ACCOUNT;
use crate::utils::message::{Content, Message};

use super::ChatroomMsg;

#[derive(Debug, Clone)]
pub(crate) struct MessageGroup {
    pub sender_id: i64,
    pub sender_name: String,
    pub messages: Vec<Message>,
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

        if &self.sender_id == ACCOUNT.get().unwrap() {
            root_box.set_halign(Align::End)
        }

        root_box
    }

    fn init_widgets(
        &mut self,
        _index: &DynamicIndex,
        root: &Self::Root,
        _returned_widget: &Widget,
        _input: &Sender<Self::Input>,
        _output: &Sender<Self::Output>,
    ) -> Self::Widgets {
        let message_alignment = if &self.sender_id == ACCOUNT.get().unwrap() {
            Align::End
        } else {
            Align::Start
        };

        relm4::view! {
            avatar_box = Box {
                set_orientation: Orientation::Vertical,
                #[name = "avatar"]
                Avatar {
                    set_size: 32,
                    set_text: Some(self.sender_name.as_str()),
                    set_show_initials: true
                }
            }
        }

        let avatar_path = get_user_avatar_path(self.sender_id);
        if avatar_path.exists() {
            if let Ok(pixbuf) = Pixbuf::from_file_at_size(avatar_path, 32, 32) {
                let image = Picture::for_pixbuf(&pixbuf);
                if let Some(paintable) = image.paintable() {
                    avatar.set_custom_image(Some(&paintable));
                }
            }
        } else {
            task::spawn(download_user_avatar_file(self.sender_id));
        }

        relm4::view! {
            main_box = Box {
                set_orientation: Orientation::Vertical,
                set_spacing: 4,
                #[name = "username_label"]
                Label {
                    set_label: &self.sender_name,
                    set_css_classes: &["caption"]
                },
                #[name = "messages_box"]
                Box {
                    set_orientation: Orientation::Vertical,
                }
            }
        }

        for message in self.messages.iter() {
            // let label = message.text();
            relm4::view! {
                message_box = Box {
                    set_css_classes: &["card", "message-box"],
                    set_halign: message_alignment,
                    set_margin_all: 2,
                    #[name = "inner_message_box"]
                    Box {
                        set_css_classes: &["inner-message-box"],
                        set_margin_all: 8,
                    }
                }
            }
            for content in message.contents.clone() {
                match content {
                    Content::Text(text) => {
                        let label = Label::builder().label(&text).selectable(true).build();
                        inner_message_box.append(&label)
                    }
                    Content::Image {
                        url: _,
                        filename: _,
                    } => {
                        let label = Label::new(Some("[图片]"));
                        inner_message_box.append(&label)
                    }
                }
            }
            messages_box.append(&message_box);
        }

        if &self.sender_id == ACCOUNT.get().unwrap() {
            username_label.set_halign(Align::End);
            root.append(&main_box);
            root.append(&avatar_box);
        } else {
            username_label.set_halign(Align::Start);
            root.append(&avatar_box);
            root.append(&main_box);
        }
    }
}
