use relm4::factory::{DynamicIndex, FactoryComponent};
use relm4::{adw, gtk, Sender};

use adw::{prelude::*, ExpanderRow};
use gtk::{Box, Label, Widget};
use ricq::structs::FriendInfo;

use super::SidebarMsg;

#[derive(Debug, Clone)]
pub struct ContactGroup {
    pub id: u8,
    pub name: String,
    pub friends: Vec<FriendInfo>,
}

impl FactoryComponent<Box, SidebarMsg> for ContactGroup {
    type InitParams = ContactGroup;
    type Widgets = ();
    type Input = ();
    type Output = ();
    type Command = ();
    type CommandOutput = ();
    type Root = ExpanderRow;

    fn init_model(
        init_params: Self::InitParams,
        _index: &DynamicIndex,
        _input: &Sender<Self::Input>,
        _output: &Sender<Self::Output>,
    ) -> Self {
        init_params
    }

    fn init_root(&self) -> Self::Root {
        let subtitle = format!(
            "{} {}",
            self.friends.len(),
            if self.friends.len() == 1 {
                "Person"
            } else {
                "People"
            }
        );
        relm4::view! {
            group = ExpanderRow {
                set_width_request: 360,
                add_prefix = &Label {
                    set_label: self.name.as_str()
                },
                set_subtitle: &subtitle
            }
        }

        group
    }

    fn init_widgets(
        &mut self,
        _index: &DynamicIndex,
        root: &Self::Root,
        _returned_widget: &Widget,
        _input: &Sender<Self::Input>,
        _output: &Sender<Self::Output>,
    ) -> Self::Widgets {
    }
}
