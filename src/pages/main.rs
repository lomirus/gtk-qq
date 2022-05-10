use relm4::factory::{FactoryPrototype, FactoryVec};
use relm4::{adw, gtk, send, ComponentUpdate, Model, Sender, WidgetPlus, Widgets};

use adw::prelude::*;
use adw::{HeaderBar, Leaflet, ViewStack, ViewSwitcherTitle, Avatar};
use gtk::{Align, Box, Label, Orientation};

use crate::{AppModel, Message};

const MOCK_CHATS_LIST: [(&str, &str); 7] = [
    ("飞翔的企鹅", "Hello"),
    ("奔跑的野猪", "World"),
    ("摆烂的修勾", "World"),
    ("躺平的猫咪", "World"),
    ("想润的鼠鼠", "World"),
    ("咆哮的先辈", "World"),
    ("被填充过多并被用于测试对齐的字符串标签", "2333"),
];

pub struct MainPageModel {
    message: Option<MainMsg>,
    chats_list: FactoryVec<ChatsItem>,
}

pub enum MainMsg {
    WindowFolded,
    _AddChatsItem(String, String),
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
    type View = Box;

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

impl Model for MainPageModel {
    type Msg = MainMsg;
    type Widgets = MainPageWidgets;
    type Components = ();
}

impl ComponentUpdate<AppModel> for MainPageModel {
    fn init_model(_parent_model: &AppModel) -> Self {
        let mut chats_list = FactoryVec::<ChatsItem>::new();
        MOCK_CHATS_LIST.iter().for_each(|(username, last_message)| {
            chats_list.push(ChatsItem {
                username: username.to_string(),
                last_message: last_message.to_string(),
            });
        });
        MainPageModel {
            message: None,
            chats_list,
        }
    }

    fn update(
        &mut self,
        msg: MainMsg,
        _components: &(),
        _sender: Sender<MainMsg>,
        _parent_sender: Sender<Message>,
    ) {
        match msg {
            MainMsg::WindowFolded => self.message = Some(MainMsg::WindowFolded),
            MainMsg::_AddChatsItem(username, last_message) => self.chats_list.push(ChatsItem {
                username: username.to_string(),
                last_message: last_message.to_string(),
            }),
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
                    add_titled(Some("chats"), "Chats") = &Box {
                        set_orientation: gtk::Orientation::Vertical,
                        set_margin_all: 12,
                        set_spacing: 5,
                        factory!(model.chats_list)
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
            append = &gtk::Separator::new(Orientation::Horizontal) {
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
                    set_vexpand: true,
                    set_valign: Align::Center,
                    set_halign: Align::Center,
                    append = &Label {
                        set_label: "Chatroom"
                    },
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
            if let MainMsg::WindowFolded = message {
                self.root_widget().set_visible_child(&self.chatroom);
            }
        }
    }
}
