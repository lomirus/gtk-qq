use relm4::factory::{DynamicIndex, FactoryComponent};
use relm4::{adw, gtk, Sender};

use adw::{prelude::*, Avatar};
use gtk::{Align, Box, Label, ListBox, Orientation};

use crate::db::get_db;

use super::SidebarMsg;

#[derive(Debug)]
pub struct ChatItem {
    pub account: i64,
    pub name: String,
    pub is_group: bool,
    pub last_message: String,
}

#[relm4::factory(pub)]
impl FactoryComponent<ListBox, SidebarMsg> for ChatItem {
    type InitParams = (i64, bool, String);
    type Widgets = ChatItemWidgets;
    type Input = ();
    type Output = ();
    type Command = ();
    type CommandOutput = ();

    view! {
        root = Box {
            set_margin_top: 8,
            set_margin_bottom: 8,
            Avatar {
                set_text: Some(&self.name),
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
                    set_text: self.name.as_str(),
                    add_css_class: "heading"
                },
                #[name = "last_message"]
                Label {
                    set_text: self.last_message.as_str(),
                    add_css_class: "caption",
                    set_xalign: 0.0,
                },
            },
        }
    }

    fn init_model(
        init_params: Self::InitParams,
        _index: &DynamicIndex,
        _input: &Sender<Self::Input>,
        _output: &Sender<Self::Output>,
    ) -> Self {
        let (account, is_group, last_message) = init_params;
        let last_message = last_message.replace('\n', " ");
        let conn = get_db();
        let name = if is_group {
            conn.query_row("Select name from groups where id=?1", [account], |row| {
                row.get(0)
            })
        } else {
            conn.query_row("Select remark from friends where id=?1", [account], |row| {
                row.get(0)
            })
        }
        .unwrap_or_else(|_| {
            println!(concat!(
                "It seems that you just got a chat item without name. ",
                "Try to refresh the contact in sidebar. If the ",
                "problem still exists, please report it on Github.",
            ));
            "CHAT_ITEM_NAME".to_string()
        });
        ChatItem {
            account,
            is_group,
            name,
            last_message,
        }
    }

    fn pre_view() {
        widgets.last_message.set_label(&self.last_message);
    }
}
