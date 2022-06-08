mod friends;
mod group_item;

use relm4::factory::FactoryVecDeque;
use relm4::{
    adw, gtk, Component, ComponentController, ComponentParts, ComponentSender, Controller,
    SimpleComponent, WidgetPlus,
};
use std::cell::RefCell;

use adw::{prelude::*, ViewStack, ViewSwitcherBar};
use gtk::{Box, Button, Entry, EntryIconPosition, ListBox, Orientation, ScrolledWindow};
use tokio::task;

use friends::FriendsModel;
use crate::db::sql::{get_db, refresh_groups_list, Group};
use super::SidebarMsg;

#[derive(Debug)]
pub struct ContactModel {
    friends: Controller<FriendsModel>,
    groups_list: Option<RefCell<FactoryVecDeque<ListBox, Group, ContactMsg>>>,
    is_refresh_groups_button_enabled: bool,
}

impl ContactModel {
    fn render_groups(&self) -> rusqlite::Result<()> {
        let mut groups_list = self.groups_list.as_ref().unwrap().borrow_mut();
        groups_list.clear();

        let conn = get_db();

        let mut stmt = conn.prepare("Select id, name, owner_id from groups order by name")?;
        let groups = stmt
            .query_map([], |row| {
                Ok(Group {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    owner_id: row.get(2)?,
                })
            })?
            .map(|result| result.unwrap());

        for group in groups {
            groups_list.push_back(group);
        }

        groups_list.render_changes();
        Ok(())
    }
}

async fn refresh_groups(sender: ComponentSender<ContactModel>) {
    sender.output(SidebarMsg::PushToast(
        "Start refreshing the groups list...".to_string(),
    ));
    match refresh_groups_list().await {
        Ok(_) => sender.input(ContactMsg::RenderGroups),
        Err(err) => sender.output(SidebarMsg::PushToast(err.to_string())),
    }
}

#[derive(Debug)]
pub enum ContactMsg {
    SelectChatroom(i64, bool),
    RefreshGroups,
    RenderGroups,
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
        },
        contact_groups = Box {
            set_orientation: Orientation::Vertical,
            Box {
                set_margin_all: 8,
                Button {
                    #[watch]
                    set_sensitive: model.is_refresh_groups_button_enabled,
                    set_tooltip_text: Some("Refreshing groups list"),
                    set_icon_name: "view-refresh-symbolic",
                    set_margin_end: 8,
                    connect_clicked[sender] => move |_| {
                        sender.input(ContactMsg::RefreshGroups);
                    },
                },
                #[name = "search_groups_entry"]
                Entry {
                    set_icon_from_icon_name: (EntryIconPosition::Secondary, Some("system-search-symbolic")),
                    set_placeholder_text: Some("Search in groups..."),
                    set_width_request: 320 - 3 * 8 - 32
                },
            },
            ScrolledWindow {
                set_child: contact_groups_list = Some(&ListBox) {
                    set_css_classes: &["navigation-sidebar"],
                    set_vexpand: true,
                    connect_row_activated[sender] => move |_, selected_row| {
                        let index = selected_row.index();
                        let conn = get_db();
                        let mut stmt = conn.prepare("Select id from groups order by name").unwrap();
                        let mut group_iter = stmt.query_map([], |row| { row.get(0) }).unwrap();
                        let account = group_iter.nth(index as usize).unwrap().unwrap();
                        sender.output(SidebarMsg::SelectChatroom(account, true));
                    },
                }
            }
        }
    }

    fn init(
        _init_params: (),
        root: &Self::Root,
        sender: &ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let mut model = ContactModel {
            friends: FriendsModel::builder()
                .launch(())
                .forward(&sender.input, |message| message),
            groups_list: None,
            is_refresh_groups_button_enabled: true,
        };
        let widgets = view_output!();

        let contact_stack: &ViewStack = &widgets.contact_stack;

        let friends = contact_stack.add_titled(model.friends.widget(), None, "Friends");
        let groups = contact_stack.add_titled(&widgets.contact_groups, None, "Groups");

        friends.set_icon_name(Some("person2-symbolic"));
        groups.set_icon_name(Some("people-symbolic"));

        let groups_list: FactoryVecDeque<ListBox, Group, ContactMsg> =
            FactoryVecDeque::new(widgets.contact_groups_list.clone(), &sender.input);

        model.groups_list = Some(RefCell::new(groups_list));

        model.render_groups().unwrap();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: ContactMsg, sender: &ComponentSender<Self>) {
        use ContactMsg::*;
        match msg {
            SelectChatroom(account, is_group) => {
                sender.output(SidebarMsg::SelectChatroom(account, is_group));
            }
            RefreshGroups => {
                self.is_refresh_groups_button_enabled = false;
                task::spawn(refresh_groups(sender.clone()));
            }
            RenderGroups => {
                match self.render_groups() {
                    Ok(_) => sender.output(SidebarMsg::PushToast(
                        "Refreshed the groups list.".to_string(),
                    )),
                    Err(err) => sender.output(SidebarMsg::PushToast(err.to_string())),
                }
                self.is_refresh_groups_button_enabled = true;
            }
            PushToast(msg) => {
                sender.output(SidebarMsg::PushToast(msg));
            }
        }
    }
}
