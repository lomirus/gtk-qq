use relm4::{adw, gtk, ComponentUpdate, Model, Sender, Widgets};

use adw::prelude::*;
use adw::HeaderBar;
use gtk::{Align, Box, Label, Orientation};

use crate::{AppModel, Message};

pub struct MainPageModel;

impl Model for MainPageModel {
    type Msg = ();
    type Widgets = MainPageWidgets;
    type Components = ();
}

impl ComponentUpdate<AppModel> for MainPageModel {
    fn init_model(_parent_model: &AppModel) -> Self {
        MainPageModel
    }

    fn update(
        &mut self,
        _msg: (),
        _components: &(),
        _sender: Sender<()>,
        _parent_sender: Sender<Message>,
    ) {
    }
}

#[relm4::widget(pub)]
impl Widgets<MainPageModel, AppModel> for MainPageWidgets {
    view! {
        &adw::Leaflet {
            set_can_navigate_forward: true,
            append: sidebar = &Box {
                set_vexpand: true,
                set_width_request: 360,
                set_orientation: Orientation::Vertical,
                append = &HeaderBar {
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
            },
            append: &gtk::Separator::default(),
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
            },
        }
    }
}
