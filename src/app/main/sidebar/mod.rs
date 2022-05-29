mod chat_item;
mod friends_group;
mod group_item;

use std::cell::RefCell;

use relm4::factory::FactoryVecDeque;
use relm4::{adw, gtk, ComponentParts, ComponentSender, SimpleComponent};

use adw::{prelude::*, HeaderBar, ViewStack, ViewSwitcherBar, ViewSwitcherTitle};
use gtk::{Box, ListBox, Orientation, ScrolledWindow};

use super::MainMsg;
use crate::handler::{FRIEND_GROUP_LIST, GROUP_LIST};
use chat_item::ChatItem;

pub use self::friends_group::FriendsGroup;
use self::group_item::GroupItem;

#[derive(Debug)]
pub struct SidebarModel {
    chats_list: RefCell<FactoryVecDeque<ListBox, ChatItem, SidebarMsg>>,
    friends_list: RefCell<FactoryVecDeque<Box, FriendsGroup, SidebarMsg>>,
    groups_list: RefCell<FactoryVecDeque<ListBox, GroupItem, SidebarMsg>>,
}

impl SidebarModel {
    fn update_chat_item(&self, account: i64, is_group: bool, last_message: String) {
        let mut chats_list = self.chats_list.borrow_mut();
        for i in 0..chats_list.len() {
            let this_account = chats_list.get(i).account;
            let is_this_group = chats_list.get(i).is_group;
            if this_account == account && is_this_group == is_group {
                chats_list.swap(0, i);
                chats_list.front_mut().unwrap().last_message = last_message;
                break;
            }
        }
        chats_list.render_changes();
    }

    fn insert_chat_item(&self, account: i64, is_group: bool, last_message: String) {
        let mut chats_list = self.chats_list.borrow_mut();
        chats_list.push_front((account, is_group, last_message));
        chats_list.render_changes();
    }
}

#[derive(Debug)]
pub enum SidebarMsg {
    SelectChatroom(i32),
    UpdateChatItem(i64, bool, String),
    InsertChatItem(i64, bool, String),
    RefreshContact,
}

#[relm4::component(pub)]
impl SimpleComponent for SidebarModel {
    type Input = SidebarMsg;
    type Output = MainMsg;
    type Widgets = MainPageWidgets;
    type InitParams = ();

    view! {
        #[root]
        sidebar = &Box {
            set_vexpand: true,
            set_width_request: 320,
            set_orientation: Orientation::Vertical,
            append = &HeaderBar {
                set_show_start_title_buttons: false,
                set_show_end_title_buttons: false,
                set_title_widget = Some(&ViewSwitcherTitle) {
                    set_title: "Sidebar",
                    set_stack: Some(&stack)
                }
            },
            append: stack = &ViewStack {
                set_vexpand: true,
            }
        },
        _chats = ScrolledWindow {
            set_child: sidebar_chats = Some(&ListBox) {
                set_css_classes: &["navigation-sidebar"],
                connect_row_activated[sender] => move |_, selected_row| {
                    let index = selected_row.index();
                    sender.input(SidebarMsg::SelectChatroom(index));
                },
            }
        },
        _contact = Box {
            set_orientation: Orientation::Vertical,
            append: contact_stack = &ViewStack {
                set_vexpand: true,
            },
            append = &ViewSwitcherBar {
                set_stack: Some(&contact_stack),
                set_reveal: true
            }
        },
        _contact_friends = ScrolledWindow {
            set_child: contact_friends = Some(&Box) {
                set_orientation: Orientation::Vertical,
            }
        },
        _contact_groups = ScrolledWindow {
            set_child: contact_groups = Some(&ListBox) {
                set_css_classes: &["navigation-sidebar"],
                connect_row_activated[sender] => move |_, selected_row| {
                    let index = selected_row.index();
                    let group = GROUP_LIST.get().unwrap().get(index as usize).unwrap();
                    sender.output(MainMsg::SelectChatroom(group.uin, true));
                },
            }
        }
    }

    fn init(
        _init_params: Self::InitParams,
        root: &Self::Root,
        sender: &ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let widgets = view_output!();

        let stack: &ViewStack = &widgets.stack;
        let contact_stack: &ViewStack = &widgets.contact_stack;

        let chats = stack.add_titled(&widgets._chats, None, "Chats");
        let contact = stack.add_titled(&widgets._contact, None, "Contact");
        let friends = contact_stack.add_titled(&widgets._contact_friends, None, "Friends");
        let groups = contact_stack.add_titled(&widgets._contact_groups, None, "Groups");

        chats.set_icon_name(Some("chat-symbolic"));
        contact.set_icon_name(Some("address-book-symbolic"));
        friends.set_icon_name(Some("person2-symbolic"));
        groups.set_icon_name(Some("people-symbolic"));

        let chats_list: FactoryVecDeque<ListBox, ChatItem, SidebarMsg> =
            FactoryVecDeque::new(widgets.sidebar_chats.clone(), &sender.input);
        let friends_list: FactoryVecDeque<Box, FriendsGroup, SidebarMsg> =
            FactoryVecDeque::new(widgets.contact_friends.clone(), &sender.input);
        let groups_list: FactoryVecDeque<ListBox, GroupItem, SidebarMsg> =
            FactoryVecDeque::new(widgets.contact_groups.clone(), &sender.input);

        ComponentParts {
            model: SidebarModel {
                chats_list: RefCell::new(chats_list),
                friends_list: RefCell::new(friends_list),
                groups_list: RefCell::new(groups_list),
            },
            widgets,
        }
    }

    fn update(&mut self, msg: SidebarMsg, sender: &ComponentSender<Self>) {
        use SidebarMsg::*;
        match msg {
            SelectChatroom(index) => {
                let chat_item = self.chats_list.borrow();
                let chat_item = chat_item.get(index as usize);
                let account = chat_item.account;
                let is_group = chat_item.is_group;
                sender.output(MainMsg::SelectChatroom(account, is_group));
            }
            UpdateChatItem(account, is_group, last_message) => {
                self.update_chat_item(account, is_group, last_message)
            }
            InsertChatItem(account, is_group, last_message) => {
                self.insert_chat_item(account, is_group, last_message)
            }
            RefreshContact => {
                // Refresh friends list
                let mut friends_list = self.friends_list.borrow_mut();
                let friends_group_list = FRIEND_GROUP_LIST.get().unwrap();
                for friends_group in friends_group_list.iter() {
                    friends_list.push_back(friends_group.clone());
                }
                friends_list.render_changes();
                // Refresh groups list
                let mut groups_list = self.groups_list.borrow_mut();
                for group in GROUP_LIST.get().unwrap() {
                    groups_list.push_back(GroupItem {
                        account: group.uin,
                        name: group.name.clone(),
                    });
                }
                groups_list.render_changes();
            }
        }
        self.chats_list.borrow().render_changes();
    }
}
