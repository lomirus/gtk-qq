use relm4::factory::{DynamicIndex, FactoryComponent};
use relm4::{adw, gtk, Sender, WidgetPlus};

use adw::{prelude::*, Avatar};
use gtk::gdk_pixbuf::Pixbuf;
use gtk::{Align, Box, Label, Orientation, Picture, Widget};

use tokio::task;

use crate::db::fs::{download_user_avatar_file, get_user_avatar_path};
use crate::handler::ACCOUNT;

use super::ChatroomMsg;

#[derive(Debug, Clone)]
pub struct MessageGroup {
    pub account: i64,
    pub name: String,
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
        _index: &DynamicIndex,
        root: &Self::Root,
        _returned_widget: &Widget,
        _input: &Sender<Self::Input>,
        _output: &Sender<Self::Output>,
    ) -> Self::Widgets {
        let message_alignment = if &self.account == ACCOUNT.get().unwrap() {
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
                    set_text: Some(self.name.as_str()),
                    set_show_initials: true
                }
            }
        }

        let avatar_path = get_user_avatar_path(self.account);
        if avatar_path.exists() {
            if let Ok(pixbuf) = Pixbuf::from_file_at_size(avatar_path, 32, 32) {
                let image = Picture::for_pixbuf(&pixbuf);
                if let Some(paintable) = image.paintable() {
                    avatar.set_custom_image(Some(&paintable));
                }
            }
        } else {
            task::spawn(download_user_avatar_file(self.account));
        }

        relm4::view! {
            main_box = Box {
                set_orientation: Orientation::Vertical,
                set_spacing: 4,
                #[name = "username_box"]
                Box {
                    Label {
                        set_label: &self.name,
                        set_css_classes: &["caption"]
                    }
                },
                #[name = "messages_box"]
                Box {
                    set_orientation: Orientation::Vertical,
                }
            }
        }

        for content in self.messages.iter() {
            relm4::view! {
                message_box = Box {
                    set_css_classes: &["card", "message-box"],
                    set_halign: message_alignment,
                    set_margin_all: 2,
                    Box {
                        set_css_classes: &["inner-message-box"],
                        set_margin_all: 8,
                        Label {
                            set_label: content.as_str(),
                            set_selectable: true
                        }
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
