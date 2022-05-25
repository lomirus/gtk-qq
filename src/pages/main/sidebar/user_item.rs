use relm4::factory::{DynamicIndex, FactoryComponent};
use relm4::{adw, gtk, Sender, WidgetPlus};

use adw::{prelude::*, Avatar};
use gtk::{Align, Box, Label, ListBox, ListBoxRow, Orientation};

use super::SidebarMsg;

#[derive(Debug)]
pub struct UserItem {
    pub account: i64,
    pub username: String,
    pub last_message: String,
}

impl FactoryComponent<ListBox, SidebarMsg> for UserItem {
    type InitParams = UserItem;
    type Widgets = ();
    type Input = ();
    type Output = ();
    type Command = ();
    type CommandOutput = ();
    type Root = Box;

    fn init_model(
        init_params: Self::InitParams,
        _index: &DynamicIndex,
        _input: &Sender<Self::Input>,
        _output: &Sender<Self::Output>,
    ) -> Self {
        init_params
    }

    fn init_root(&self) -> Self::Root {
        Box::default()
    }

    fn init_widgets(
        &mut self,
        _index: &DynamicIndex,
        root: &Self::Root,
        _returned_widget: &ListBoxRow,
        _input: &Sender<Self::Input>,
        _output: &Sender<Self::Output>,
    ) -> Self::Widgets {
        relm4::view! {
            item = Box {
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

        root.append(&item);
    }
}
