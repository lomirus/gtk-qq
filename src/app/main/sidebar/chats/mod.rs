mod chat_item;

use relm4::factory::FactoryVecDeque;
use relm4::{adw, gtk, ComponentParts, ComponentSender, SimpleComponent};

use adw::prelude::*;
use gtk::{ListBox, ScrolledWindow};

use super::SidebarMsg;
use chat_item::ChatItem;

#[derive(Debug)]
pub struct ChatsModel {
    chats_list: FactoryVecDeque<ListBox, ChatItem, ChatsMsg>,
}

impl ChatsModel {
    fn update_chat_item(&mut self, account: i64, is_group: bool, last_message: String) {
        for i in 0..self.chats_list.len() {
            let this_account = self.chats_list.get(i).account;
            let is_this_group = self.chats_list.get(i).is_group;
            if this_account == account && is_this_group == is_group {
                self.chats_list.swap(0, i);
                self.chats_list.front_mut().unwrap().last_message = last_message;
                break;
            }
        }
        self.chats_list.render_changes();
    }

    fn insert_chat_item(&mut self, account: i64, is_group: bool, last_message: String) {
        self.chats_list
            .push_front((account, is_group, last_message));
        self.chats_list.render_changes();
    }
}

#[derive(Debug)]
pub enum ChatsMsg {
    SelectChatroom(i32),
    UpdateChatItem(i64, bool, String),
    InsertChatItem(i64, bool, String),
}

#[relm4::component(pub)]
impl SimpleComponent for ChatsModel {
    type Input = ChatsMsg;
    type Output = SidebarMsg;
    type Widgets = ChatsWidgets;
    type InitParams = ();

    view! {
        #[root]
        chats = ScrolledWindow {
            set_child: sidebar_chats = Some(&ListBox) {
                set_css_classes: &["navigation-sidebar"],
                connect_row_activated[sender] => move |_, selected_row| {
                    let index = selected_row.index();
                    sender.input(ChatsMsg::SelectChatroom(index));
                },
            }
        }
    }

    fn init(
        _init_params: (),
        root: &Self::Root,
        sender: &ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let widgets = view_output!();

        let chats_list: FactoryVecDeque<ListBox, ChatItem, ChatsMsg> =
            FactoryVecDeque::new(widgets.sidebar_chats.clone(), &sender.input);

        let model = ChatsModel { chats_list };

        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: ChatsMsg, sender: &ComponentSender<Self>) {
        use ChatsMsg::*;
        match msg {
            SelectChatroom(index) => {
                let chat_item = self.chats_list.get(index as usize);
                let account = chat_item.account;
                let is_group = chat_item.is_group;
                sender.output(SidebarMsg::SelectChatroom(account, is_group));
            }
            UpdateChatItem(account, is_group, last_message) => {
                self.update_chat_item(account, is_group, last_message)
            }
            InsertChatItem(account, is_group, last_message) => {
                self.insert_chat_item(account, is_group, last_message)
            }
        }
    }
}
