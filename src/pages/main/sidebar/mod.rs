#![allow(unused_variables)]

mod user_item;

pub use user_item::UserItem;

use std::cell::RefCell;

use relm4::factory::FactoryVecDeque;
use relm4::{adw, gtk, ComponentParts, ComponentSender, SimpleComponent};

use adw::{prelude::*, HeaderBar, ViewStack, ViewSwitcherTitle};
use gtk::{Align, Box, Label, ListBox, Orientation, ScrolledWindow};

use ricq::msg::elem::RQElem;
use ricq::structs::FriendMessage;

use crate::handler::FRIEND_LIST;
use crate::pages::main::MainMsg;

#[derive(Debug)]
pub struct SidebarModel {
    chats_list: RefCell<FactoryVecDeque<ListBox, UserItem, SidebarMsg>>,
}

#[derive(Debug)]
pub enum SidebarMsg {
    SelectChatroom(i32),
    UpdateChatItem(FriendMessage),
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
            set_width_request: 360,
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
        contact_stack = &Box {
            set_halign: Align::Center,
            append: &Label::new(Some("Contact"))
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

        let chats_list: FactoryVecDeque<ListBox, UserItem, SidebarMsg> =
            FactoryVecDeque::new(widgets.sidebar_chats.clone(), &sender.input);

        ComponentParts {
            model: SidebarModel {
                chats_list: RefCell::new(chats_list),
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
                // Check if the sender is already in the chat list
                // if yes, just push the message into it and put it at the first place
                // if not, push the new sender to the list
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
                    chats_list.push_front(UserItem {
                        account,
                        username: user.remark.clone(),
                        last_message: content,
                    });
                }
            }
        }
        self.chats_list.borrow().render_changes();
    }
}
