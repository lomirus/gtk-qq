mod chatroom;
mod sidebar;

use std::cell::RefCell;
use std::collections::VecDeque;

use once_cell::sync::OnceCell;
use relm4::factory::FactoryVecDeque;
use relm4::{adw, gtk, ComponentParts, ComponentSender, SimpleComponent};
use ricq::msg::elem::RQElem;
use ricq::structs::FriendMessage;

use adw::{prelude::*, HeaderBar, Leaflet, ViewStack, ViewSwitcherTitle};
use gtk::{Align, Box, Label, ListBox, MenuButton, Orientation, ScrolledWindow, Separator, Stack};

use self::{chatroom::Chatroom, sidebar::UserItem};
use crate::app::AppMessage;
use crate::pages::main::chatroom::ChatroomInitParams;

pub static MAIN_SENDER: OnceCell<ComponentSender<MainPageModel>> = OnceCell::new();

#[derive(Debug)]
pub struct MainPageModel {
    message: Option<ViewMsg>,
    chats_list: RefCell<FactoryVecDeque<ListBox, UserItem, MainMsg>>,
    chatrooms: RefCell<FactoryVecDeque<Stack, Chatroom, MainMsg>>,
}

#[derive(Clone, Debug)]
pub struct Message {
    author: String,
    message: String,
}

#[derive(Debug)]
pub enum MainMsg {
    WindowFolded,
    UpdateChatItem(FriendMessage),
    SelectChatroom(i32),
}

#[derive(Debug)]
enum ViewMsg {
    WindowFolded,
    SelectChatroom(i64),
}

relm4::new_action_group!(WindowActionGroup, "menu");
relm4::new_stateless_action!(ShortcutsAction, WindowActionGroup, "shortcuts");
relm4::new_stateless_action!(AboutAction, WindowActionGroup, "about");

#[relm4::component(pub)]
impl SimpleComponent for MainPageModel {
    type Input = MainMsg;
    type Output = AppMessage;
    type Widgets = MainPageWidgets;
    type InitParams = ();

    view! {
        #[root]
        main_page = &Leaflet {
            append: sidebar = &Box {
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
            append = &Separator::new(Orientation::Horizontal) {
            },
            append: chatroom = &Box {
                set_vexpand: true,
                set_hexpand: true,
                set_orientation: Orientation::Vertical,
                append = &HeaderBar {
                    set_title_widget = Some(&Label) {
                        set_label: "Chatroom"
                    },
                    pack_end = &MenuButton {
                        set_icon_name: "menu-symbolic",
                        set_menu_model: Some(&main_menu),
                    }
                },
                append: chatroom_stack = &Stack {},
            },
            connect_folded_notify[sender] => move |leaflet| {
                if leaflet.is_folded() {
                    sender.input(MainMsg::WindowFolded);
                }
            },
        },
        chats_stack = ScrolledWindow {
            set_child: sidebar_chats = Some(&ListBox) {
                set_css_classes: &["navigation-sidebar"],
                connect_row_activated[sender] => move |_, selected_row| {
                    let index = selected_row.index();
                    sender.input(MainMsg::SelectChatroom(index));
                },
            }
        },
        contact_stack = &Box {
            set_halign: Align::Center,
            append: &Label::new(Some("Contact"))
        }
    }

    menu! {
        main_menu: {
            "Keyboard Shortcuts" => ShortcutsAction,
            "About Gtk QQ" => AboutAction
        }
    }

    fn init(
        _init_params: Self::InitParams,
        root: &Self::Root,
        sender: &ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        MAIN_SENDER
            .set(sender.clone())
            .expect("failed to initialize main sender");
        let widgets = view_output!();

        let stack: &ViewStack = &widgets.stack;
        let chats_stack = stack.add_titled(&widgets.chats_stack, None, "Chats");
        let contact_stack = stack.add_titled(&widgets.contact_stack, None, "Contact");
        chats_stack.set_icon_name(Some("chat-symbolic"));
        contact_stack.set_icon_name(Some("address-book-symbolic"));

        let chats_list: FactoryVecDeque<ListBox, UserItem, MainMsg> =
            FactoryVecDeque::new(widgets.sidebar_chats.clone(), &sender.input);
        let chatrooms: FactoryVecDeque<Stack, Chatroom, MainMsg> =
            FactoryVecDeque::new(widgets.chatroom_stack.clone(), &sender.input);

        ComponentParts {
            model: MainPageModel {
                message: None,
                chats_list: RefCell::new(chats_list),
                chatrooms: RefCell::new(chatrooms),
            },
            widgets,
        }
    }

    fn update(&mut self, msg: MainMsg, _sender: &ComponentSender<Self>) {
        use MainMsg::*;
        match msg {
            WindowFolded => self.message = Some(ViewMsg::WindowFolded),
            SelectChatroom(index) => {
                let account = self.chats_list.borrow().get(index as usize).account;
                self.message = Some(ViewMsg::SelectChatroom(account));
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
                let mut has_sender_already_in_list = false;
                let mut chats_list = self.chats_list.borrow_mut();
                let mut chatrooms = self.chatrooms.borrow_mut();
                for i in 0..chats_list.len() {
                    let this_account = chats_list.get(i).account;
                    if this_account == account {
                        has_sender_already_in_list = true;
                        chats_list.swap(0, i);
                        chats_list.front_mut().unwrap().last_message = content.clone();
                        chatrooms.swap(0, i);
                        chatrooms.front_mut().unwrap().add_message(Message {
                            author: account.to_string(),
                            message: content.to_string(),
                        });
                        break;
                    }
                }

                if !has_sender_already_in_list {
                    chats_list.push_front(UserItem {
                        account,
                        username: account.to_string(),
                        last_message: content.to_string(),
                    });
                    let mut messages = VecDeque::new();
                    messages.push_back(Message {
                        author: account.to_string(),
                        message: content,
                    });
                    chatrooms.push_front(ChatroomInitParams { account, messages });
                }
            }
        }
        self.chats_list.borrow().render_changes();
        self.chatrooms.borrow().render_changes();
    }

    fn pre_view() {
        if let Some(message) = &model.message {
            use ViewMsg::*;
            match message {
                WindowFolded => widgets.main_page.set_visible_child(&widgets.chatroom),
                SelectChatroom(id) => widgets
                    .chatroom_stack
                    .set_visible_child_name(id.to_string().as_str()),
            }
        }

        self.chatrooms.borrow().render_changes();
    }
}
