mod chats;
mod contact;

use relm4::{
    adw, gtk, Component, ComponentController, ComponentParts, ComponentSender, Controller,
    SimpleComponent,
};

use adw::{prelude::*, HeaderBar, ViewStack, ViewSwitcherTitle};
use gtk::{Box, Orientation};

use super::MainMsg;
use chats::{ChatsModel, ChatsMsg};
use contact::ContactModel;

#[derive(Debug)]
pub struct SidebarModel {
    chats: Controller<ChatsModel>,
    contact: Controller<ContactModel>,
}

#[derive(Debug)]
pub enum SidebarMsg {
    SelectChatroom(i64, bool),
    UpdateChatItem(i64, bool, String),
    InsertChatItem(i64, bool, String),
    PushToast(String),
}

#[relm4::component(pub)]
impl SimpleComponent for SidebarModel {
    type Input = SidebarMsg;
    type Output = MainMsg;
    type Widgets = SiderbarWidgets;
    type InitParams = ();

    view! {
        #[root]
        sidebar = &Box {
            set_vexpand: true,
            set_width_request: 320,
            set_orientation: Orientation::Vertical,
            HeaderBar {
                set_show_start_title_buttons: false,
                set_show_end_title_buttons: false,
                set_title_widget = Some(&ViewSwitcherTitle) {
                    set_title: "Sidebar",
                    set_stack: Some(&stack)
                }
            },
            #[name = "stack"]
            ViewStack {
                set_vexpand: true,
            }
        }
    }

    fn init(
        _init_params: (),
        root: &Self::Root,
        sender: &ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = SidebarModel {
            chats: ChatsModel::builder()
                .launch(())
                .forward(&sender.input, |message| message),
            contact: ContactModel::builder()
                .launch(())
                .forward(&sender.input, |message| message),
        };
        let widgets = view_output!();

        let stack: &ViewStack = &widgets.stack;

        let chats = stack.add_titled(model.chats.widget(), None, "Chats");
        let contact = stack.add_titled(model.contact.widget(), None, "Contact");

        chats.set_icon_name(Some("chat-symbolic"));
        contact.set_icon_name(Some("address-book-symbolic"));

        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: SidebarMsg, sender: &ComponentSender<Self>) {
        use SidebarMsg::*;
        match msg {
            SelectChatroom(account, is_group) => {
                sender.output(MainMsg::SelectChatroom(account, is_group));
            }
            UpdateChatItem(account, is_group, last_message) => {
                self.chats
                    .sender()
                    .send(ChatsMsg::UpdateChatItem(account, is_group, last_message));
            }
            InsertChatItem(account, is_group, last_message) => {
                self.chats
                    .sender()
                    .send(ChatsMsg::InsertChatItem(account, is_group, last_message));
            }
            PushToast(message) => sender.output(MainMsg::PushToast(message)),
        }
    }
}
