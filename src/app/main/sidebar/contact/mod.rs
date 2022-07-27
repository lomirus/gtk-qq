mod friends;
mod groups;

use relm4::{
    adw, component::Controller, gtk, Component, ComponentController, ComponentParts,
    ComponentSender, SimpleComponent,
};

use adw::{prelude::*, ViewStack, ViewSwitcherBar};
use gtk::{Box, Orientation};

use self::groups::GroupsModel;
use friends::FriendsModel;

use super::SidebarMsg;

#[derive(Debug)]
pub struct ContactModel {
    friends: Controller<FriendsModel>,
    groups: Controller<GroupsModel>,
}

#[derive(Debug)]
pub enum ContactMsg {
    SelectChatroom(i64, bool),
    PushToast(String),
}

#[relm4::component(pub)]
impl SimpleComponent for ContactModel {
    type Input = ContactMsg;
    type Output = SidebarMsg;
    type Widgets = ContactWIdgets;
    type InitParams = ();

    view! {
        #[root]
        contact = Box {
            set_orientation: Orientation::Vertical,
            #[name = "contact_stack"]
            ViewStack {
                set_vexpand: true,
            },
            ViewSwitcherBar {
                set_stack: Some(&contact_stack),
                set_reveal: true
            }
        }
    }

    fn init(
        _init_params: (),
        root: &Self::Root,
        sender: &ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = ContactModel {
            friends: FriendsModel::builder()
                .launch(())
                .forward(&sender.input, |message| message),
            groups: GroupsModel::builder()
                .launch(())
                .forward(&sender.input, |message| message),
        };
        let widgets = view_output!();

        let contact_stack: &ViewStack = &widgets.contact_stack;

        let friends = contact_stack.add_titled(model.friends.widget(), None, "Friends");
        let groups = contact_stack.add_titled(model.groups.widget(), None, "Groups");

        friends.set_icon_name(Some("person2-symbolic"));
        groups.set_icon_name(Some("people-symbolic"));

        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: ContactMsg, sender: &ComponentSender<Self>) {
        use ContactMsg::*;
        match msg {
            SelectChatroom(account, is_group) => {
                sender.output(SidebarMsg::SelectChatroom(account, is_group));
            }
            PushToast(msg) => {
                sender.output(SidebarMsg::PushToast(msg));
            }
        }
    }
}
