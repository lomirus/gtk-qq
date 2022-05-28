mod chat_item;
mod contact_group;

use std::cell::RefCell;

use relm4::factory::FactoryVecDeque;
use relm4::{adw, gtk, ComponentParts, ComponentSender, SimpleComponent};

use adw::{prelude::*, HeaderBar, ViewStack, ViewSwitcherTitle};
use gtk::{Box, ListBox, Orientation, ScrolledWindow};

use crate::handler::FRIEND_GROUP_LIST;
use super::MainMsg;
use chat_item::ChatItem;

pub use self::contact_group::ContactGroup;

#[derive(Debug)]
pub struct SidebarModel {
    chats_list: RefCell<FactoryVecDeque<ListBox, ChatItem, SidebarMsg>>,
    contact_list: RefCell<FactoryVecDeque<Box, ContactGroup, SidebarMsg>>,
}

impl SidebarModel {
    fn update_chat_item(&self, account: i64, last_message: String) {
        let mut chats_list = self.chats_list.borrow_mut();
        for i in 0..chats_list.len() {
            let this_account = chats_list.get(i).account;
            if this_account == account {
                chats_list.swap(0, i);
                chats_list.front_mut().unwrap().last_message = last_message;
                break;
            }
        }
        chats_list.render_changes();
    }

    fn insert_chat_item(&self, account: i64, last_message: String) {
        let mut chats_list = self.chats_list.borrow_mut();
        chats_list.push_front((account, last_message));
        chats_list.render_changes();
    }
}

#[derive(Debug)]
pub enum SidebarMsg {
    SelectChatroom(i32),
    UpdateChatItem(i64, String),
    InsertChatItem(i64, String),
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
        _chats_stack = ScrolledWindow {
            set_child: sidebar_chats = Some(&ListBox) {
                set_css_classes: &["navigation-sidebar"],
                connect_row_activated[sender] => move |_, selected_row| {
                    let index = selected_row.index();
                    sender.input(SidebarMsg::SelectChatroom(index));
                },
            }
        },
        _contact_stack = ScrolledWindow {
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
        let chats_stack = stack.add_titled(&widgets._chats_stack, None, "Chats");
        let contact_stack = stack.add_titled(&widgets._contact_stack, None, "Contact");
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
            UpdateChatItem(account, last_message) => self.update_chat_item(account, last_message),
            InsertChatItem(account, last_message) => self.insert_chat_item(account, last_message),
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
