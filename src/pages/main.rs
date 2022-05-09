use relm4::{adw, gtk, send, ComponentUpdate, Model, Sender, Widgets};

use adw::prelude::*;
use adw::HeaderBar;
use gtk::{Align, Box, Label, Orientation};

use crate::{AppModel, Message};

pub struct MainPageModel {
    message: MainMsg,
}

pub enum MainMsg {
    None,
    FoldedChange,
}

impl Model for MainPageModel {
    type Msg = MainMsg;
    type Widgets = MainPageWidgets;
    type Components = ();
}

impl ComponentUpdate<AppModel> for MainPageModel {
    fn init_model(_parent_model: &AppModel) -> Self {
        MainPageModel {
            message: MainMsg::None,
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
            MainMsg::None => (),
            MainMsg::FoldedChange => self.message = MainMsg::FoldedChange,
        }
    }
}

#[relm4::widget(pub)]
impl Widgets<MainPageModel, AppModel> for MainPageWidgets {
    view! {
        &adw::Leaflet {
            append: sidebar = &Box {
                set_vexpand: true,
                set_width_request: 360,
                set_orientation: Orientation::Vertical,
                append = &HeaderBar {
                    set_show_start_title_buttons: false,
                    set_show_end_title_buttons: false,
                    set_title_widget = Some(&Label) {
                        set_label: "Contact"
                    },
                },
                append = &Box {
                    set_vexpand: true,
                    set_valign: Align::Center,
                    set_halign: Align::Center,
                    append = &Label {
                        set_label: "Sidebar"
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
            connect_folded_notify(sender) => move |_| {
                send!(sender, MainMsg::FoldedChange);
            },
        }
    }

    fn pre_view() {
        match model.message {
            MainMsg::None => (),
            MainMsg::FoldedChange => {
                self.root_widget().set_visible_child(&self.chatroom);
            }
        };
    }
}
