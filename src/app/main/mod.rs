mod chatroom;
mod sidebar;

use std::cell::RefCell;
use std::collections::VecDeque;

use once_cell::sync::OnceCell;
use relm4::factory::FactoryVecDeque;
use relm4::{
    adw, gtk, Component, ComponentController, ComponentParts, ComponentSender, Controller,
    SimpleComponent,
};

use adw::{prelude::*, HeaderBar, Leaflet};
use gtk::{Align, Box, Label, MenuButton, Orientation, Separator, Stack};

use crate::handler::{ACCOUNT, FRIEND_LIST};
use chatroom::{Chatroom, ChatroomInitParams};
pub use sidebar::ContactGroup;
use sidebar::{SidebarModel, SidebarMsg};

pub static MAIN_SENDER: OnceCell<ComponentSender<MainPageModel>> = OnceCell::new();

#[derive(Debug)]
pub struct MainPageModel {
    message: Option<ViewMsg>,
    sidebar: Controller<SidebarModel>,
    chatrooms: RefCell<FactoryVecDeque<Stack, Chatroom, MainMsg>>,
}

impl MainPageModel {
    fn is_user_in_list(&self, account: i64) -> bool {
        let chatrooms = self.chatrooms.borrow();

        for i in 0..chatrooms.len() {
            let chatroom = chatrooms.get(i);
            if chatroom.account == account {
                return true;
            }
        }

        false
    }

    fn insert_chatroom(&self, account: i64) {
        // TODO: Get history messages
        let messages = VecDeque::new();
        let mut chatrooms = self.chatrooms.borrow_mut();
        chatrooms.push_front(ChatroomInitParams { account, messages });
    }

    fn push_own_message(&self, target: i64, content: String) {
        let self_account = *ACCOUNT.get().unwrap();
        let mut chatrooms = self.chatrooms.borrow_mut();
        for i in 0..chatrooms.len() {
            let mut chatroom = chatrooms.get_mut(i);
            if chatroom.account == target {
                chatroom.push_message(Message {
                    sender: self_account,
                    target,
                    content,
                });
                break;
            }
        }
    }

    fn push_others_message(&self, sender: i64, content: String) {
        let self_account = *ACCOUNT.get().unwrap();
        let mut chatrooms = self.chatrooms.borrow_mut();
        for i in 0..chatrooms.len() {
            let mut chatroom = chatrooms.get_mut(i);
            if chatroom.account == sender {
                chatroom.push_message(Message {
                    sender,
                    target: self_account,
                    content,
                });
                break;
            }
        }
    }
}

#[derive(Clone, Debug)]
pub struct Message {
    pub sender: i64,
    pub target: i64,
    pub content: String,
}

#[derive(Debug)]
pub enum MainMsg {
    WindowFolded,
    ReceiveMessage(i64, String),
    SendMessage(i64, String),
    SelectChatroom(i64),
    InitSidebar,
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
    type Output = ();
    type Widgets = MainPageWidgets;
    type InitParams = ();

    view! {
        #[root]
        main_page = &Leaflet {
            append: sidebar_controller.widget(),
            append = &Separator::new(Orientation::Horizontal),
            append: chatroom = &Box {
                set_vexpand: true,
                set_hexpand: true,
                set_orientation: Orientation::Vertical,
                append = &HeaderBar {
                    set_title_widget = Some(&Box) {
                        set_orientation: Orientation::Vertical,
                        set_valign: Align::Center,
                        append: chatroom_title = &Label {
                            set_label: "Chatroom"
                        },
                        append: chatroom_subtitle = &Label {
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

        let sidebar_controller = SidebarModel::builder()
            .launch(())
            .forward(&sender.input, |message| message);

        let widgets = view_output!();

        let chatrooms: FactoryVecDeque<Stack, Chatroom, MainMsg> =
            FactoryVecDeque::new(widgets.chatroom_stack.clone(), &sender.input);

        ComponentParts {
            model: MainPageModel {
                message: None,
                sidebar: sidebar_controller,
                chatrooms: RefCell::new(chatrooms),
            },
            widgets,
        }
    }

    fn update(&mut self, msg: MainMsg, _sender: &ComponentSender<Self>) {
        use MainMsg::*;
        match msg {
            WindowFolded => self.message = Some(ViewMsg::WindowFolded),
            SelectChatroom(account) => {
                if !self.is_user_in_list(account) {
                    // TODO: Get last_message from history or some other places
                    self.sidebar
                        .sender()
                        .send(SidebarMsg::InsertChatItem(account, String::new()));
                    self.insert_chatroom(account)
                }

                self.message = Some(ViewMsg::SelectChatroom(account));
            }
            ReceiveMessage(sender, content) => {
                // Update components
                use SidebarMsg::*;
                if self.is_user_in_list(sender) {
                    self.sidebar
                        .sender()
                        .send(UpdateChatItem(sender, content.clone()));
                } else {
                    self.sidebar
                        .sender()
                        .send(InsertChatItem(sender, content.clone()));
                    self.insert_chatroom(sender);
                    // 当所插入的 chatroom 为唯一的一个 chatroom 时，将其设为焦点，
                    // 以触发自动更新 chatroom 的标题与副标题。
                    if self.chatrooms.borrow().len() == 1 {
                        self.message = Some(ViewMsg::SelectChatroom(sender));
                    }
                }

                self.push_others_message(sender, content);
            }
            SendMessage(target, content) => {
                use SidebarMsg::*;
                if self.is_user_in_list(target) {
                    self.sidebar
                        .sender()
                        .send(UpdateChatItem(target, content.clone()));
                } else {
                    self.sidebar
                        .sender()
                        .send(InsertChatItem(target, content.clone()));
                    self.insert_chatroom(target);
                    // 当所插入的 chatroom 为唯一的一个 chatroom 时，将其设为焦点，
                    // 以触发自动更新 chatroom 的标题与副标题。
                    if self.chatrooms.borrow().len() == 1 {
                        self.message = Some(ViewMsg::SelectChatroom(target));
                    }
                }
                self.push_own_message(target, content);
            }
            InitSidebar => {
                self.sidebar.sender().send(SidebarMsg::RefreshContact);
            }
        }
        self.chatrooms.borrow().render_changes();
    }

    fn pre_view() {
        if let Some(message) = &model.message {
            use ViewMsg::*;
            match message {
                WindowFolded => widgets.main_page.set_visible_child(&widgets.chatroom),
                SelectChatroom(account) => {
                    chatroom_stack.set_visible_child_name(&account.to_string());
                    let user = FRIEND_LIST
                        .get()
                        .unwrap()
                        .iter()
                        .find(|user| user.uin == *account)
                        .unwrap();
                    let title = &user.remark;
                    let subtitle = format!("{} ({})", user.nick, account);
                    chatroom_title.set_label(title);
                    chatroom_subtitle.set_label(&subtitle);
                }
            }
        }
    }
}
