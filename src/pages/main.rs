use relm4::factory::{positions::StackPageInfo, FactoryPrototype, FactoryVec};
use relm4::{adw, gtk, send, ComponentUpdate, Model, Sender, WidgetPlus, Widgets};

use adw::prelude::*;
use adw::{Avatar, HeaderBar, Leaflet, ViewStack, ViewSwitcherTitle};
use gtk::{
    Align, Box, Button, Entry, Label, ListBox, Orientation, ScrolledWindow, Separator, Stack,
};

use crate::{AppModel, Message};

const MOCK_CHATS_LIST: [(&str, &str); 13] = [
    ("飞翔的企鹅", "Hello"),
    ("奔跑的野猪", "World"),
    ("摆烂的修勾", "喵喵"),
    ("躺平的猫咪", "汪汪"),
    ("想润的鼠鼠", "鼠鼠我啊"),
    ("咆哮的先辈", "哼哼"),
    ("叛逆的鲁路", "2333"),
    ("死亡的笔记", "2333"),
    ("进击的巨人", "2333"),
    ("炼金的术士", "2333"),
    ("忧郁的凉宫", "2333"),
    ("灼眼的夏娜", "2333"),
    ("科学的磁炮", "2333"),
    // ("被填充过多并被用于测试文本对齐和溢出的字符串标签", "2333"),
];

pub struct MainPageModel {
    message: Option<MainMsg>,
    chats_list: FactoryVec<ChatsItem>,
    chatrooms: FactoryVec<Chatroom>,
}

pub enum MainMsg {
    WindowFolded,
    SelectChatroom(i32),
}

struct ChatsItem {
    username: String,
    last_message: String,
}

#[relm4::factory_prototype]
impl FactoryPrototype for ChatsItem {
    type Factory = FactoryVec<Self>;
    type Widgets = ChatsItemWidgets;
    type Msg = MainMsg;
    type View = ListBox;

    view! {
        Box {
            append = &Avatar {
                set_text: Some(&self.username),
                set_show_initials: true,
                set_size: 56
            },
            append = &Box {
                set_margin_all: 8,
                set_orientation: Orientation::Vertical,
                set_halign: Align::Center,
                set_spacing: 8,
                append = &Label {
                    set_text: self.username.as_str(),
                    add_css_class: "heading"
                },
                append = &Label {
                    set_text: self.last_message.as_str(),
                    add_css_class: "caption",
                    set_xalign: 0.0,
                },
            },
        }
    }

    fn position(&self, _index: &usize) {}
}

struct Chatroom {
    username: String,
    messages: Vec<String>,
}

#[relm4::factory_prototype]
impl FactoryPrototype for Chatroom {
    type Factory = FactoryVec<Self>;
    type Widgets = ChatroomWidgets;
    type Msg = MainMsg;
    type View = Stack;

    view! {
        Box {
            append = &Label {
                set_text: args!(format!("{}: ", self.username).as_str()),
            },
            append = &Label {
                set_text: self.messages.join(", ").as_str(),
            },
        }
    }

    fn position(&self, index: &usize) -> StackPageInfo {
        StackPageInfo {
            name: Some(index.to_string()),
            title: Some(index.to_string()),
        }
    }
}

impl Model for MainPageModel {
    type Msg = MainMsg;
    type Widgets = MainPageWidgets;
    type Components = ();
}

impl ComponentUpdate<AppModel> for MainPageModel {
    fn init_model(_parent_model: &AppModel) -> Self {
        let mut chats_list = FactoryVec::<ChatsItem>::new();
        let mut chatrooms = FactoryVec::<Chatroom>::new();
        MOCK_CHATS_LIST.iter().for_each(|(username, last_message)| {
            chats_list.push(ChatsItem {
                username: username.to_string(),
                last_message: last_message.to_string(),
            });
            chatrooms.push(Chatroom {
                username: username.to_string(),
                messages: vec![last_message.to_string()],
            });
        });
        MainPageModel {
            message: None,
            chats_list,
            chatrooms,
        }
    }

    fn update(
        &mut self,
        msg: MainMsg,
        _components: &(),
        _sender: Sender<MainMsg>,
        _parent_sender: Sender<Message>,
    ) {
        use MainMsg::*;
        match msg {
            WindowFolded => self.message = Some(MainMsg::WindowFolded),
            SelectChatroom(id) => self.message = Some(MainMsg::SelectChatroom(id)),
        }
    }
}

#[relm4::widget(pub)]
impl Widgets<MainPageModel, AppModel> for MainPageWidgets {
    view! {
        &Leaflet {
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
                    add_titled(Some("chats"), "Chats") = &ScrolledWindow {
                        set_child = Some(&ListBox) {
                            set_css_classes: &["navigation-sidebar"],
                            connect_row_activated(sender) => move |_, selected_row| {
                                let index = selected_row.index();
                                send!(sender, MainMsg::SelectChatroom(index))
                            },
                            factory!(model.chats_list)
                        }
                    } -> {
                        set_icon_name: Some("chat-symbolic")
                    },
                    add_titled(Some("contact"), "Contact") = &Box {
                        set_halign: Align::Center,
                        append: &Label::new(Some("Contact"))
                    } -> {
                        set_icon_name: Some("address-book-symbolic")
                    },
                }
            } -> {
                set_navigatable: true
            },
            append = &Separator::new(Orientation::Horizontal) {
            } -> {
                set_navigatable: false
            },
            append: chatroom = &Box {
                set_vexpand: true,
                set_hexpand: true,
                set_orientation: Orientation::Vertical,
                append = &HeaderBar {
                    set_title_widget = Some(&Label) {
                        set_label: "Chatroom"
                    },
                },
                append = &Box {
                    set_orientation: Orientation::Vertical,
                    append: chatroom_stack = &Stack {
                        set_vexpand: true,
                        set_halign: Align::Center,
                        factory!(model.chatrooms)
                    },
                    append = &Box {
                        set_margin_all: 8,
                        append = &Entry {
                            set_hexpand: true,
                            set_show_emoji_icon: true,
                            set_placeholder_text: Some("Send a message..."),
                            set_margin_end: 8
                        },
                        append = &Button {
                            set_icon_name: "send-symbolic",
                        },
                    }
                }
            } -> {
                set_navigatable: true
            },
            connect_folded_notify(sender) => move |leaflet| {
                if leaflet.is_folded() {
                    send!(sender, MainMsg::WindowFolded);
                }
            },
        }
    }

    fn pre_view() {
        if let Some(message) = &model.message {
            use MainMsg::*;
            match message {
                WindowFolded => self.root_widget().set_visible_child(&self.chatroom),
                SelectChatroom(id) => self
                    .chatroom_stack
                    .set_visible_child_name(id.to_string().as_str()),
            }
        }
    }
}
