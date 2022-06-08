mod friends_group;

use relm4::factory::FactoryVecDeque;
use relm4::{adw, gtk, ComponentParts, ComponentSender, SimpleComponent, WidgetPlus};
use std::cell::RefCell;

use adw::prelude::*;
use gtk::{Box, Button, Entry, EntryIconPosition, Orientation, ScrolledWindow};
use tokio::task;

use super::ContactMsg;
use crate::db::sql::{get_db, refresh_friends_list, Friend};
use friends_group::FriendsGroup;

#[derive(Debug)]
pub struct FriendsModel {
    friends_list: Option<RefCell<FactoryVecDeque<Box, FriendsGroup, FriendsMsg>>>,
    is_refresh_button_enabled: bool,
    keywords: String
}

impl FriendsModel {
    fn render_friends(&self) -> rusqlite::Result<()> {
        let mut friends_list = self.friends_list.as_ref().unwrap().borrow_mut();
        friends_list.clear();

        let conn = get_db();

        let mut stmt = conn.prepare("Select id, name, remark, group_id from friends")?;
        let friends: Vec<Friend> = stmt
            .query_map([], |row| {
                Ok(Friend {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    remark: row.get(2)?,
                    group_id: row.get(3)?,
                })
            })?
            .map(|result| result.unwrap())
            .collect();

        let friends_groups: Vec<FriendsGroup> = conn
            .prepare("Select id, name, online_friends from friends_groups")?
            .query_map([], |row| {
                Ok(FriendsGroup {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    online_friends: row.get(2)?,
                    friends: friends
                        .clone()
                        .into_iter()
                        .filter(|friend| friend.group_id == row.get(0).unwrap())
                        .collect(),
                })
            })?
            .map(|result| result.unwrap())
            .collect();

        for friends_group in friends_groups {
            friends_list.push_back(friends_group);
        }

        friends_list.render_changes();

        Ok(())
    }
}

async fn refresh_friends(sender: ComponentSender<FriendsModel>) {
    sender.output(ContactMsg::PushToast(
        "Start refreshing the friends list...".to_string(),
    ));
    match refresh_friends_list().await {
        Ok(_) => sender.input(FriendsMsg::RenderFriends),
        Err(err) => sender.output(ContactMsg::PushToast(err.to_string())),
    }
}

#[derive(Debug)]
pub enum FriendsMsg {
    SelectChatroom(i64, bool),
    SearchFriends(String),
    RefreshFriends,
    RenderFriends,
}

#[relm4::component(pub)]
impl SimpleComponent for FriendsModel {
    type Input = FriendsMsg;
    type Output = ContactMsg;
    type Widgets = FriendsWidgets;
    type InitParams = ();

    view! {
        #[root]
        contact_friends = Box {
            set_orientation: Orientation::Vertical,
            Box {
                set_margin_all: 8,
                Button {
                    #[watch]
                    set_sensitive: model.is_refresh_button_enabled,
                    set_tooltip_text: Some("Refresh friends list"),
                    set_icon_name: "view-refresh-symbolic",
                    set_margin_end: 8,
                    connect_clicked[sender] => move |_| {
                        sender.input(FriendsMsg::RefreshFriends);
                    },
                },
                #[name = "search_entry"]
                Entry {
                    set_icon_from_icon_name: (EntryIconPosition::Secondary, Some("system-search-symbolic")),
                    set_placeholder_text: Some("Search in friends..."),
                    set_width_request: 320 - 3 * 8 - 32,
                    connect_changed[sender] => move |entry| {
                        let keywords = entry.buffer().text();
                        sender.input(FriendsMsg::SearchFriends(keywords));
                    },
                },
            },
            ScrolledWindow {
                set_child: friends_list = Some(&Box) {
                    set_vexpand: true,
                    set_orientation: Orientation::Vertical,
                }
            }
        }
    }

    fn init(
        _init_params: (),
        root: &Self::Root,
        sender: &ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let mut model = FriendsModel {
            friends_list: None,
            is_refresh_button_enabled: true,
            keywords: String::new()
        };
        let widgets = view_output!();

        let friends_list: FactoryVecDeque<Box, FriendsGroup, FriendsMsg> =
            FactoryVecDeque::new(widgets.friends_list.clone(), &sender.input);

        model.friends_list = Some(RefCell::new(friends_list));

        model.render_friends().unwrap();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: FriendsMsg, sender: &ComponentSender<Self>) {
        use FriendsMsg::*;
        match msg {
            SelectChatroom(account, is_group) => {
                sender.output(ContactMsg::SelectChatroom(account, is_group));
            }
            RefreshFriends => {
                self.is_refresh_button_enabled = false;
                task::spawn(refresh_friends(sender.clone()));
            }
            RenderFriends => {
                match self.render_friends() {
                    Ok(_) => sender.output(ContactMsg::PushToast(
                        "Refreshed the friends list.".to_string(),
                    )),
                    Err(err) => sender.output(ContactMsg::PushToast(err.to_string())),
                }
                self.is_refresh_button_enabled = true;
            }
            SearchFriends(keywords) => {
                println!("{keywords}");
                self.keywords = keywords;
            }
        }
    }
}
