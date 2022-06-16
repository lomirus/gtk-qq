mod chatroom;
mod sidebar;

use std::cell::RefCell;
use std::collections::VecDeque;

use once_cell::sync::OnceCell;
use relm4::factory::FactoryVecDeque;
use relm4::{
    adw, gtk, Component, ComponentController, ComponentParts, ComponentSender, Controller,
};

use adw::{prelude::*, HeaderBar, Leaflet, Toast, ToastOverlay};
use gtk::{Align, Box, Label, MenuButton, Orientation, Separator, Stack};

use chatroom::{Chatroom, ChatroomInitParams};
use sidebar::{SidebarModel, SidebarMsg};

use crate::db::sql::{get_db, get_group_name};

pub static MAIN_SENDER: OnceCell<ComponentSender<MainPageModel>> = OnceCell::new();

#[derive(Debug)]
pub struct MainPageModel {
    sidebar: Controller<SidebarModel>,
    chatrooms: RefCell<FactoryVecDeque<Stack, Chatroom, MainMsg>>,
}

impl MainPageModel {
    fn is_item_in_list(&self, account: i64, is_group: bool) -> bool {
        let chatrooms = self.chatrooms.borrow();

        for i in 0..chatrooms.len() {
            let chatroom = chatrooms.get(i);
            if chatroom.account == account && chatroom.is_group == is_group {
                return true;
            }
        }

        false
    }

    fn insert_chatroom(&self, account: i64, is_group: bool) {
        // TODO: Get history messages
        let messages = VecDeque::new();
        let mut chatrooms = self.chatrooms.borrow_mut();
        chatrooms.push_front(ChatroomInitParams {
            account,
            is_group,
            messages,
        });

        chatrooms.render_changes();
    }

    fn push_friend_message(&self, friend_id: i64, message: Message) {
        let mut chatrooms = self.chatrooms.borrow_mut();
        for i in 0..chatrooms.len() {
            let mut chatroom = chatrooms.get_mut(i);
            if chatroom.account == friend_id && !chatroom.is_group {
                chatroom.push_message(message);
                break;
            }
        }
    }

    fn push_group_message(&self, group_id: i64, message: Message) {
        let mut chatrooms = self.chatrooms.borrow_mut();
        for i in 0..chatrooms.len() {
            let mut chatroom = chatrooms.get_mut(i);
            if chatroom.account == group_id && chatroom.is_group {
                chatroom.push_message(message);
                break;
            }
        }
    }
}

#[derive(Clone, Debug)]
pub struct Message {
    pub sender_id: i64,
    pub sender_name: String,
    pub content: String,
}

#[derive(Debug)]
pub enum MainMsg {
    WindowFolded,
    GroupMessage { group_id: i64, message: Message },
    FriendMessage { friend_id: i64, message: Message },
    SelectChatroom(i64, bool),
    PushToast(String),
}

pub struct MainPageWidgets {
    root: ToastOverlay,
    main_page: Leaflet,
    chatroom: Box,
    chatroom_title: Label,
    chatroom_subtitle: Label,
    chatroom_stack: Stack,
}

relm4::new_action_group!(WindowActionGroup, "menu");
relm4::new_stateless_action!(ShortcutsAction, WindowActionGroup, "shortcuts");
relm4::new_stateless_action!(AboutAction, WindowActionGroup, "about");

impl Component for MainPageModel {
    type Input = MainMsg;
    type Output = ();
    type Widgets = MainPageWidgets;
    type InitParams = ();
    type Root = ToastOverlay;
    type CommandOutput = ();

    fn init_root() -> Self::Root {
        ToastOverlay::new()
    }

    fn init(
        _init_params: Self::InitParams,
        root: &Self::Root,
        sender: &ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        MAIN_SENDER
            .set(sender.clone())
            .expect("failed to initialize main sender");

        let sidebar_controller = SidebarModel::builder()
            .launch(())
            .forward(&sender.input, |message| message);

        relm4::menu! {
            main_menu: {
                "Keyboard Shortcuts" => ShortcutsAction,
                "About Gtk QQ" => AboutAction
            }
        }

        relm4::view! {
            #[name = "main_page"]
            Leaflet {
                append: sidebar_controller.widget(),
                append = &Separator::new(Orientation::Horizontal),
                #[name = "chatroom"]
                append = &Box {
                    set_vexpand: true,
                    set_hexpand: true,
                    set_orientation: Orientation::Vertical,
                    HeaderBar {
                        set_title_widget = Some(&Box) {
                            set_orientation: Orientation::Vertical,
                            set_valign: Align::Center,
                            #[name = "chatroom_title"]
                            Label {
                                set_label: "Chatroom"
                            },
                            #[name = "chatroom_subtitle"]
                            Label {
                                set_css_classes: &["subtitle"],
                                set_label: "Chatroom"
                            },
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
            }
        }

        root.set_child(Some(&main_page));

        let chatrooms: FactoryVecDeque<Stack, Chatroom, MainMsg> =
            FactoryVecDeque::new(chatroom_stack.clone(), &sender.input);

        ComponentParts {
            model: MainPageModel {
                sidebar: sidebar_controller,
                chatrooms: RefCell::new(chatrooms),
            },
            widgets: MainPageWidgets {
                root: root.clone(),
                main_page,
                chatroom,
                chatroom_title,
                chatroom_subtitle,
                chatroom_stack,
            },
        }
    }

    fn update_with_view(
        &mut self,
        widgets: &mut Self::Widgets,
        msg: Self::Input,
        _sender: &ComponentSender<Self>,
    ) {
        use MainMsg::*;
        match msg {
            WindowFolded => {
                widgets.main_page.set_visible_child(&widgets.chatroom);
            }
            SelectChatroom(account, is_group) => {
                if !self.is_item_in_list(account, is_group) {
                    // TODO: Get last_message from history or some other places
                    self.sidebar.sender().send(SidebarMsg::InsertChatItem(
                        account,
                        is_group,
                        String::new(),
                    ));
                    self.insert_chatroom(account, is_group);
                }

                let child_name =
                    &format!("{} {}", account, if is_group { "group" } else { "friend" });
                widgets.chatroom_stack.set_visible_child_name(child_name);

                if is_group {
                    let group_name: String = get_group_name(account);
                    let title = group_name;
                    let subtitle = account.to_string();
                    widgets.chatroom_title.set_label(&title);
                    widgets.chatroom_subtitle.set_label(&subtitle);
                } else {
                    let (user_name, user_remark): (String, String) = get_db()
                        .query_row(
                            "Select name, remark from friends where id=?1",
                            [account],
                            |row| Ok((row.get(0).unwrap(), row.get(1).unwrap())),
                        )
                        .unwrap();
                    let title = &user_name;
                    let subtitle = format!("{} ({})", user_remark, account);
                    widgets.chatroom_title.set_label(title);
                    widgets.chatroom_subtitle.set_label(&subtitle);
                }
            }
            FriendMessage { friend_id, message } => {
                use SidebarMsg::*;
                if self.is_item_in_list(friend_id, false) {
                    self.sidebar.sender().send(UpdateChatItem(
                        friend_id,
                        false,
                        message.content.clone(),
                    ));
                } else {
                    self.sidebar.sender().send(InsertChatItem(
                        friend_id,
                        false,
                        message.content.clone(),
                    ));
                    self.insert_chatroom(friend_id, false);
                    // 当所插入的 chatroom 为唯一的一个 chatroom 时，将其设为焦点，
                    // 以触发自动更新 chatroom 的标题与副标题。
                    if self.chatrooms.borrow().len() == 1 {
                        let child_name = &format!("{} friend", friend_id);
                        widgets.chatroom_stack.set_visible_child_name(child_name);

                        let (user_name, user_remark): (String, String) = get_db()
                            .query_row(
                                "Select name, remark from friends where id=?1",
                                [friend_id],
                                |row| Ok((row.get(0).unwrap(), row.get(1).unwrap())),
                            )
                            .unwrap();
                        let title = &user_name;
                        let subtitle = format!("{} ({})", user_remark, friend_id);
                        widgets.chatroom_title.set_label(title);
                        widgets.chatroom_subtitle.set_label(&subtitle);
                    }
                }

                self.push_friend_message(friend_id, message);
            }
            GroupMessage { group_id, message } => {
                use SidebarMsg::*;
                if self.is_item_in_list(group_id, true) {
                    self.sidebar.sender().send(UpdateChatItem(
                        group_id,
                        true,
                        message.content.clone(),
                    ));
                } else {
                    self.sidebar.sender().send(InsertChatItem(
                        group_id,
                        true,
                        message.content.clone(),
                    ));
                    self.insert_chatroom(group_id, true);
                    // 当所插入的 chatroom 为唯一的一个 chatroom 时，将其设为焦点，
                    // 以触发自动更新 chatroom 的标题与副标题。
                    if self.chatrooms.borrow().len() == 1 {
                        let child_name = &format!("{} group", group_id);
                        widgets.chatroom_stack.set_visible_child_name(child_name);

                        let group_name: String = get_group_name(group_id);
                        let title = group_name;
                        let subtitle = group_id.to_string();
                        widgets.chatroom_title.set_label(&title);
                        widgets.chatroom_subtitle.set_label(&subtitle);
                    }
                }

                self.push_group_message(group_id, message);
            }
            PushToast(content) => {
                widgets.root.add_toast(&Toast::new(&content));
            }
        }
    }
}
