#![allow(unused_variables)]

mod chat_item;
mod contact_group;

use std::cell::RefCell;

use relm4::factory::FactoryVecDeque;
use relm4::{adw, gtk, ComponentParts, ComponentSender, SimpleComponent};

use adw::{prelude::*, HeaderBar, ViewStack, ViewSwitcherTitle};
use gtk::{Box, ListBox, Orientation, ScrolledWindow};

use ricq::msg::elem::RQElem;
use ricq::structs::FriendMessage;

use crate::handler::{FRIEND_GROUP_LIST, FRIEND_LIST};
use crate::pages::main::MainMsg;
use chat_item::ChatItem;

pub use self::contact_group::ContactGroup;

#[derive(Debug)]
pub struct SidebarModel {
    chats_list: RefCell<FactoryVecDeque<ListBox, ChatItem, SidebarMsg>>,
    contact_list: RefCell<FactoryVecDeque<Box, ContactGroup, SidebarMsg>>,
}

#[derive(Debug)]
pub enum SidebarMsg {
    SelectChatroom(i32),
    UpdateChatItem(FriendMessage),
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
        chats_stack = ScrolledWindow {
            set_child: sidebar_chats = Some(&ListBox) {
                set_css_classes: &["navigation-sidebar"],
                connect_row_activated[sender] => move |_, selected_row| {
                    let index = selected_row.index();
                    sender.input(SidebarMsg::SelectChatroom(index));
                },
            }
        },
        contact_stack = ScrolledWindow {
            set_child: sidebar_contact = Some(&Box) {
                set_orientation: Orientation::Vertical,
                // set_css_classes: &["navigation-sidebar"]
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
        let chats_stack = stack.add_titled(&widgets.chats_stack, None, "Chats");
        let contact_stack = stack.add_titled(&widgets.contact_stack, None, "Contact");
        chats_stack.set_icon_name(Some("chat-symbolic"));
        contact_stack.set_icon_name(Some("address-book-symbolic"));

        let chats_list: FactoryVecDeque<ListBox, ChatItem, SidebarMsg> =
            FactoryVecDeque::new(widgets.sidebar_chats.clone(), &sender.input);
        let contact_list: FactoryVecDeque<Box, ContactGroup, SidebarMsg> =
            FactoryVecDeque::new(widgets.sidebar_contact.clone(), &sender.input);

        ComponentParts {
            model: SidebarModel {
                chats_list: RefCell::new(chats_list),
                contact_list: RefCell::new(contact_list),
            },
            widgets,
        }
    }

    fn update(&mut self, msg: SidebarMsg, sender: &ComponentSender<Self>) {
        use SidebarMsg::*;
        match msg {
            SelectChatroom(index) => {
                let account = self.chats_list.borrow().get(index as usize).account;
                sender.output(MainMsg::SelectChatroom(account));
            }
            UpdateChatItem(message) => {
                // Get sender account
                let account = message.from_uin;
                // Get message content
                let mut content = String::new();
                for elem in message.elements {
                    if let RQElem::Text(text) = elem {
                        content = text.content;
                    }
                }
                // Check if the sender is already in the chat list.
                // if yes, just push the message into it and put it at the first place.
                // if not, push the new sender to the list.
                let mut has_sender_already_in_list = false;
                let mut chats_list = self.chats_list.borrow_mut();
                for i in 0..chats_list.len() {
                    let this_account = chats_list.get(i).account;
                    if this_account == account {
                        has_sender_already_in_list = true;
                        chats_list.swap(0, i);
                        chats_list.front_mut().unwrap().last_message = content.clone();
                        break;
                    }
                }
                if !has_sender_already_in_list {
                    let user = FRIEND_LIST
                        .get()
                        .unwrap()
                        .iter()
                        .find(|user| user.uin == account)
                        .unwrap();
                    chats_list.push_front(ChatItem {
                        account,
                        username: user.remark.clone(),
                        last_message: content,
                    });
                }
            }
            RefreshContact => {
                let mut contact_list = self.contact_list.borrow_mut();
                let friend_group_list = FRIEND_GROUP_LIST.get().unwrap();
                for group in friend_group_list.iter() {
                    contact_list.push_back(group.clone());
                }
                contact_list.render_changes();
            }
        }
        self.chats_list.borrow().render_changes();
    }
}
