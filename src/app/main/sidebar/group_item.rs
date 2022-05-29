use relm4::factory::{DynamicIndex, FactoryComponent};
use relm4::{adw, gtk, Sender};

use adw::{prelude::*, Avatar};
use gtk::{Align, Box, Label, ListBox, ListBoxRow, Orientation};

use super::SidebarMsg;

#[derive(Debug)]
pub struct GroupItem {
    pub account: i64,
    pub name: String,
}

impl FactoryComponent<ListBox, SidebarMsg> for GroupItem {
    type InitParams = GroupItem;
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
                set_margin_top: 8,
                set_margin_bottom: 8,
                append = &Avatar {
                    set_text: Some(&self.name),
                    set_show_initials: true,
                    set_size: 48,
                    set_margin_end: 8
                },
                append = &Box {
                    set_orientation: Orientation::Vertical,
                    set_halign: Align::Center,
                    set_spacing: 8,
                    append = &Label {
                        set_text: self.name.as_str(),
                        add_css_class: "heading"
                    },
                    append = &Label {
                        set_text: self.account.to_string().as_str(),
                        add_css_class: "caption",
                        set_xalign: 0.0,
                    },
                },
            }
        }

        root.append(&item);
    }
}
