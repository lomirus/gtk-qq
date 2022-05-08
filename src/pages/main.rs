use relm4::{adw, gtk, ComponentUpdate, Model, Sender, Widgets};

use adw::HeaderBar;
use gtk::{Align, Box, Label, Orientation};

use adw::prelude::*;

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
        &Box {
            set_hexpand: true,
            set_vexpand: true,
            set_orientation: Orientation::Vertical,
            append = &HeaderBar {
                set_title_widget = Some(&Label) {
                    set_label: "GTK4 QQ"
                },
            },
            append = &Box {
                set_halign: Align::Center,
                set_valign: Align::Center,
                set_vexpand: true,
                set_orientation: Orientation::Vertical,
                append = &Box {
                    append = &Label {
                        set_label: "Hello, World!"
                    },
                }
            }
        }
    }
}
