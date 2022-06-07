use relm4::factory::{DynamicIndex, FactoryComponent};
use relm4::{adw, gtk, Sender};

use adw::{prelude::*, Avatar};
use gtk::gdk_pixbuf::Pixbuf;
use gtk::pango::EllipsizeMode;
use gtk::{Align, Box, Label, ListBox, ListBoxRow, Orientation, Picture};

use tokio::task;

use crate::db::{
    fs::{
        download_group_avatar_file, download_user_avatar_file, get_group_avatar_path,
        get_user_avatar_path,
    },
    sql::{get_friend_remark, get_group_name},
};

use super::SidebarMsg;

#[derive(Debug)]
pub struct ChatItem {
    pub account: i64,
    pub name: String,
    pub is_group: bool,
    pub last_message: String,
}

pub struct ChatItemWidgets {
    pub last_message: Label,
}

impl FactoryComponent<ListBox, SidebarMsg> for ChatItem {
    type InitParams = (i64, bool, String);
    type Widgets = ChatItemWidgets;
    type Input = ();
    type Output = ();
    type Command = ();
    type CommandOutput = ();
    type Root = Box;

    fn init_root(&self) -> Self::Root {
        relm4::view! {
            root = Box {
                set_margin_top: 8,
                set_margin_bottom: 8,
            }
        }

        root
    }

    fn init_widgets(
        &mut self,
        _index: &DynamicIndex,
        root: &Self::Root,
        _returned_widget: &ListBoxRow,
        _input: &Sender<Self::Input>,
        _output: &Sender<Self::Output>,
    ) -> Self::Widgets {
        relm4::view! {
            #[name = "avatar"]
            Avatar {
                set_text: Some(&self.name),
                set_show_initials: true,
                set_size: 48,
                set_margin_end: 8
            }
        };

        if self.is_group {
            let avatar_path = get_group_avatar_path(self.account);
            if avatar_path.exists() {
                if let Ok(pixbuf) = Pixbuf::from_file_at_size(avatar_path, 48, 48) {
                    let image = Picture::for_pixbuf(&pixbuf);
                    if let Some(paintable) = image.paintable() {
                        avatar.set_custom_image(Some(&paintable));
                    }
                }
            } else {
                task::spawn(download_group_avatar_file(self.account));
            }
        } else {
            let avatar_path = get_user_avatar_path(self.account);
            if avatar_path.exists() {
                if let Ok(pixbuf) = Pixbuf::from_file_at_size(avatar_path, 48, 48) {
                    let image = Picture::for_pixbuf(&pixbuf);
                    if let Some(paintable) = image.paintable() {
                        avatar.set_custom_image(Some(&paintable));
                    }
                }
            } else {
                task::spawn(download_user_avatar_file(self.account));
            }
        }

        relm4::view! {
            #[name = "info"]
            Box {
                set_orientation: Orientation::Vertical,
                set_halign: Align::Start,
                set_spacing: 8,
                Label {
                    set_xalign: 0.0,
                    set_text: self.name.as_str(),
                    set_ellipsize: EllipsizeMode::End,
                    add_css_class: "heading"
                },
                #[name = "last_message"]
                Label {
                    set_text: self.last_message.as_str(),
                    set_ellipsize: EllipsizeMode::End,
                    add_css_class: "caption",
                    set_xalign: 0.0,
                }
            }
        };

        root.append(&avatar);
        root.append(&info);

        ChatItemWidgets { last_message }
    }

    fn init_model(
        init_params: Self::InitParams,
        _index: &DynamicIndex,
        _input: &Sender<Self::Input>,
        _output: &Sender<Self::Output>,
    ) -> Self {
        let (account, is_group, last_message) = init_params;
        let last_message = last_message.replace('\n', " ");
        let name = if is_group {
            get_group_name(account)
        } else {
            get_friend_remark(account)
        };
        ChatItem {
            account,
            is_group,
            name,
            last_message,
        }
    }

    fn update_view(
        &self,
        widgets: &mut Self::Widgets,
        _input: &Sender<Self::Input>,
        _output: &Sender<Self::Output>,
    ) {
        widgets.last_message.set_label(&self.last_message);
    }
}
