use relm4::factory::{FactoryPrototype, FactoryVec};
use relm4::{adw, gtk, WidgetPlus};

use adw::prelude::*;
use adw::Avatar;
use gtk::{Align, Box, Label, ListBox, Orientation};

use super::MainMsg;

pub struct ChatsItem {
    pub username: String,
    pub last_message: String,
}

#[relm4::factory_prototype(pub)]
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
