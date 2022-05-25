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
use gtk::{Box, Label, MenuButton, Orientation, Separator, Stack};

use ricq::msg::elem::RQElem;
use ricq::structs::FriendMessage;

use self::chatroom::Chatroom;
use self::sidebar::SidebarModel;
use crate::pages::main::chatroom::ChatroomInitParams;
use crate::pages::main::sidebar::SidebarMsg;

pub static MAIN_SENDER: OnceCell<ComponentSender<MainPageModel>> = OnceCell::new();

#[derive(Debug)]
pub struct MainPageModel {
    message: Option<ViewMsg>,
    sidebar: Controller<SidebarModel>,
    chatrooms: RefCell<FactoryVecDeque<Stack, Chatroom, MainMsg>>,
}

#[derive(Clone, Debug)]
pub struct Message {
    account: i64,
    message: String,
}

#[derive(Debug)]
pub enum MainMsg {
    WindowFolded,
    UpdateChatItem(FriendMessage),
    SelectChatroom(i64),
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
    type Output = MainMsg;
    type Widgets = MainPageWidgets;
    type InitParams = ();

    view! {
        #[root]
        main_page = &Leaflet {
            append: sidebar_controller.widget(),
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
                self.message = Some(ViewMsg::SelectChatroom(account));
            }
            UpdateChatItem(message) => {
                // Get sender account
                let account = message.from_uin;
                // Get message content
                let mut content = String::new();
                for elem in message.elements.clone() {
                    if let RQElem::Text(text) = elem {
                        content = text.content;
                    }
                }
                // Check if the sender is already in the chat list
                // if yes, just push the message into it and put it at the first place
                // if not, push the new sender to the list and create a new chatroom
                self.sidebar
                    .sender()
                    .send(SidebarMsg::UpdateChatItem(message));
                let mut has_sender_already_in_list = false;
                let mut chatrooms = self.chatrooms.borrow_mut();

                for i in 0..chatrooms.len() {
                    let mut chatroom = chatrooms.get_mut(i);
                    if chatroom.account == account {
                        has_sender_already_in_list = true;
                        chatroom.add_message(Message {
                            account,
                            message: content.to_string(),
                        });
                        break;
                    }
                }
                if !has_sender_already_in_list {
                    let mut messages = VecDeque::new();
                    messages.push_back(Message {
                        account,
                        message: content,
                    });
                    chatrooms.push_front(ChatroomInitParams { account, messages });
                }
            }
        }
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
