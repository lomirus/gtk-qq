use relm4::factory::{DynamicIndex, FactoryComponent};
use relm4::{adw, gtk, Sender, WidgetPlus};

use adw::{prelude::*, Avatar, ExpanderRow};
use gtk::gdk_pixbuf::Pixbuf;
use gtk::glib::clone;
use gtk::pango::EllipsizeMode;
use gtk::{Align, Box, GestureClick, Label, Orientation, Picture, Widget};

use tokio::task;

use super::FriendsMsg;
use crate::db::fs::{download_user_avatar_file, get_user_avatar_path};
use crate::db::sql::Friend;

pub enum FriendsGroupMessage {
    SelectUser(i64),
}

#[derive(Debug, Clone)]
pub struct FriendsGroup {
    pub id: u8,
    pub name: String,
    pub online_friends: i32,
    pub friends: Vec<Friend>,
}

impl FactoryComponent<Box, FriendsMsg> for FriendsGroup {
    type InitParams = FriendsGroup;
    type Widgets = ();
    type Input = FriendsGroupMessage;
    type Output = FriendsMsg;
    type Command = ();
    type CommandOutput = ();
    type Root = ExpanderRow;

    fn init_model(
        init_params: Self::InitParams,
        _index: &DynamicIndex,
        _input: &Sender<Self::Input>,
        _output: &Sender<Self::Output>,
    ) -> Self {
        init_params
    }

    fn init_root(&self) -> Self::Root {
        let subtitle = format!("{}/{}", self.online_friends, self.friends.len());
        relm4::view! {
            group = ExpanderRow {
                set_width_request: 320,
                add_prefix = &Label {
                    set_label: self.name.as_str()
                },
                set_subtitle: &subtitle
            }
        }

        group
    }

    fn init_widgets(
        &mut self,
        _index: &DynamicIndex,
        group: &Self::Root,
        _returned_widget: &Widget,
        input: &Sender<Self::Input>,
        _output: &Sender<Self::Output>,
    ) -> Self::Widgets {
        let friends = self.friends.clone();
        for friend in friends.into_iter() {
            // Create user item click event
            let gesture = GestureClick::new();
            gesture.connect_released(clone!(@strong input => move |_, _, _, _| {
                input.send(FriendsGroupMessage::SelectUser(friend.id));
            }));

            relm4::view! {
                child = Box {
                    set_margin_all: 8,
                    #[name = "avatar"]
                    Avatar {
                        set_text: Some(&friend.name),
                        set_show_initials: true,
                        set_size: 48,
                        set_margin_end: 8
                    },
                    Box {
                        set_orientation: Orientation::Vertical,
                        set_halign: Align::Start,
                        set_spacing: 8,
                        Label {
                            set_xalign: 0.0,
                            set_text:  &friend.remark,
                            add_css_class: "heading",
                            set_ellipsize: EllipsizeMode::End,
                        },
                        Label {
                            set_text: &friend.name,
                            add_css_class: "caption",
                            set_xalign: 0.0,
                            set_ellipsize: EllipsizeMode::End,
                        },
                    },
                    add_controller: &gesture,
                }
            }

            let avatar_path = get_user_avatar_path(friend.id);
            if avatar_path.exists() {
                if let Ok(pixbuf) = Pixbuf::from_file_at_size(avatar_path, 48, 48) {
                    let image = Picture::for_pixbuf(&pixbuf);
                    if let Some(paintable) = image.paintable() {
                        avatar.set_custom_image(Some(&paintable));
                    }
                }
            } else {
                task::spawn(download_user_avatar_file(friend.id));
            }

            group.add_row(&child);
        }
    }

    fn update(
        &mut self,
        relm_msg: Self::Input,
        _input: &Sender<Self::Input>,
        output: &Sender<Self::Output>,
    ) -> Option<Self::Command> {
        use FriendsGroupMessage::*;
        match relm_msg {
            SelectUser(account) => {
                output.send(FriendsMsg::SelectChatroom(account, false));
            }
        }
        None
    }

    fn output_to_parent_msg(output: FriendsMsg) -> Option<FriendsMsg> {
        Some(output)
    }
}
