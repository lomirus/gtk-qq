mod group_item;

use relm4::factory::FactoryVecDeque;
use relm4::{adw, gtk, ComponentParts, ComponentSender, SimpleComponent, WidgetPlus};
use std::cell::RefCell;

use adw::prelude::*;
use gtk::{Box, Button, Entry, EntryIconPosition, ListBox, Orientation, ScrolledWindow};
use tokio::task;

use super::ContactMsg;
use crate::db::sql::{get_db, refresh_groups_list, Group};

#[derive(Debug)]
pub struct GroupsModel {
    groups_list: Option<RefCell<FactoryVecDeque<ListBox, Group, GroupsMsg>>>,
    is_refresh_button_enabled: bool,
    keywords: String,
}

impl GroupsModel {
    fn render_groups(&self) -> rusqlite::Result<()> {
        let mut groups_list = self.groups_list.as_ref().unwrap().borrow_mut();
        groups_list.clear();

        let conn = get_db();

        let mut stmt = conn.prepare("Select id, name from groups order by name")?;
        let groups = stmt
            .query_map([], |row| {
                Ok(Group {
                    id: row.get(0)?,
                    name: row.get(1)?,
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

async fn refresh_groups(sender: ComponentSender<GroupsModel>) {
    sender.output(ContactMsg::PushToast(
        "Start refreshing the groups list...".to_string(),
    ));
    match refresh_groups_list().await {
        Ok(_) => sender.input(GroupsMsg::Render),
        Err(err) => sender.output(ContactMsg::PushToast(err.to_string())),
    }
}

#[derive(Debug)]
pub enum GroupsMsg {
    Refresh,
    Render,
    Search(String),
}

#[relm4::component(pub)]
impl SimpleComponent for GroupsModel {
    type Input = GroupsMsg;
    type Output = ContactMsg;
    type Widgets = ContactWIdgets;
    type InitParams = ();

    view! {
        #[root]
        groups = Box {
            set_orientation: Orientation::Vertical,
            Box {
                set_margin_all: 8,
                Button {
                    #[watch]
                    set_sensitive: model.is_refresh_button_enabled,
                    set_tooltip_text: Some("Refresh groups list"),
                    set_icon_name: "view-refresh-symbolic",
                    set_margin_end: 8,
                    connect_clicked[sender] => move |_| {
                        sender.input(GroupsMsg::Refresh);
                    },
                },
                #[name = "search_entry"]
                Entry {
                    set_icon_from_icon_name: (EntryIconPosition::Secondary, Some("system-search-symbolic")),
                    set_placeholder_text: Some("Search in groups..."),
                    set_width_request: 320 - 3 * 8 - 32,
                    connect_changed[sender] => move |entry| {
                        let keywords = entry.buffer().text();
                        sender.input(GroupsMsg::Search(keywords));
                    },
                },
            },
            ScrolledWindow {
                set_child: groups_list = Some(&ListBox) {
                    set_css_classes: &["navigation-sidebar"],
                    set_vexpand: true,
                    connect_row_activated[sender] => move |_, selected_row| {
                        let index = selected_row.index();
                        let conn = get_db();
                        let mut stmt = conn.prepare("Select id from groups order by name").unwrap();
                        let mut group_iter = stmt.query_map([], |row| { row.get(0) }).unwrap();
                        let account = group_iter.nth(index as usize).unwrap().unwrap();
                        sender.output(ContactMsg::SelectChatroom(account, true));
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
        let mut model = GroupsModel {
            groups_list: None,
            is_refresh_button_enabled: true,
            keywords: String::new(),
        };
        let widgets = view_output!();

        let groups_list: FactoryVecDeque<ListBox, Group, GroupsMsg> =
            FactoryVecDeque::new(widgets.groups_list.clone(), &sender.input);

        model.groups_list = Some(RefCell::new(groups_list));

        model.render_groups().unwrap();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: GroupsMsg, sender: &ComponentSender<Self>) {
        use GroupsMsg::*;
        match msg {
            Refresh => {
                self.is_refresh_button_enabled = false;
                task::spawn(refresh_groups(sender.clone()));
            }
            Render => {
                match self.render_groups() {
                    Ok(_) => sender.output(ContactMsg::PushToast(
                        "Refreshed the groups list.".to_string(),
                    )),
                    Err(err) => sender.output(ContactMsg::PushToast(err.to_string())),
                }
                self.is_refresh_button_enabled = true;
            }
            Search(keywords) => {
                println!("{keywords}");
                self.keywords = keywords;
            }
        }
    }
}
