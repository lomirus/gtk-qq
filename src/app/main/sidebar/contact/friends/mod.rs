mod friends_group;
mod search_item;

use tokio::task;

use relm4::factory::FactoryVecDeque;
use relm4::{adw, gtk, Component, ComponentParts, ComponentSender, WidgetPlus};

use adw::prelude::*;
use gtk::{Box, Button, Entry, EntryIconPosition, ListBox, Orientation, ScrolledWindow};

use super::ContactMsg;
use crate::db::sql::{get_db, refresh_friends_list, Friend};
use friends_group::FriendsGroup;

#[derive(Debug)]
pub struct FriendsModel {
    friends_list: Option<FactoryVecDeque<Box, FriendsGroup, FriendsMsg>>,
    search_list: Option<FactoryVecDeque<ListBox, Friend, FriendsMsg>>,
    is_refresh_button_enabled: bool,
}

impl FriendsModel {
    fn render_friends(&mut self) -> rusqlite::Result<()> {
        let friends_list = self.friends_list.as_mut().unwrap();
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

    fn render_search_result(&mut self, keyword: String) -> rusqlite::Result<()> {
        let search_list = self.search_list.as_mut().unwrap();
        search_list.clear();

        let keyword = keyword.to_lowercase();
        let conn = get_db();

        let mut stmt = conn.prepare("Select id, name, remark, group_id from friends")?;
        let eligible_friends: Vec<Friend> = stmt
            .query_map([], |row| {
                Ok(Friend {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    remark: row.get(2)?,
                    group_id: row.get(3)?,
                })
            })?
            .map(|result| result.unwrap())
            .filter(|friend| {
                let match_name = friend.name.to_lowercase().contains(&keyword);
                let match_remark = friend.remark.to_lowercase().contains(&keyword);

                match_name || match_remark
            })
            .collect();

        for friend in eligible_friends {
            search_list.push_back(friend);
        }

        search_list.render_changes();

        Ok(())
    }
}

async fn refresh_friends(sender: ComponentSender<FriendsModel>) {
    sender.output(ContactMsg::PushToast(
        "Start refreshing the friends list...".to_string(),
    ));
    match refresh_friends_list().await {
        Ok(_) => sender.input(FriendsMsg::Render),
        Err(err) => sender.output(ContactMsg::PushToast(err.to_string())),
    }
}

#[derive(Debug)]
pub enum FriendsMsg {
    SelectChatroom(i64, bool),
    SelectSearchItem(i32),
    Search(String),
    Refresh,
    Render,
}

#[derive(Debug)]
pub struct FriendsWidgets {
    friend_list: Box,
    search_list: ListBox,
    scrolled_window: ScrolledWindow,
}

impl Component for FriendsModel {
    type Input = FriendsMsg;
    type Output = ContactMsg;
    type Widgets = FriendsWidgets;
    type InitParams = ();
    type Root = Box;
    type CommandOutput = ();

    fn init_root() -> Box {
        Box::new(Orientation::Vertical, 0)
    }

    fn init(
        _init_params: (),
        root: &Self::Root,
        sender: &ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let mut model = FriendsModel {
            friends_list: None,
            search_list: None,
            is_refresh_button_enabled: true,
        };

        relm4::view! {
            #[name = "header_bar"]
            Box {
                set_margin_all: 8,
                Button {
                    #[watch]
                    set_sensitive: model.is_refresh_button_enabled,
                    set_tooltip_text: Some("Refresh friends list"),
                    set_icon_name: "view-refresh-symbolic",
                    set_margin_end: 8,
                    connect_clicked[sender] => move |_| {
                        sender.input(FriendsMsg::Refresh);
                    },
                },
                #[name = "search_entry"]
                Entry {
                    set_icon_from_icon_name: (EntryIconPosition::Secondary, Some("system-search-symbolic")),
                    set_placeholder_text: Some("Search in friends..."),
                    set_width_request: 320 - 3 * 8 - 32,
                    connect_changed[sender] => move |entry| {
                        let keywords = entry.buffer().text();
                        sender.input(FriendsMsg::Search(keywords));
                    },
                },
            },
            #[name = "scrolled_window"]
            ScrolledWindow {
                set_child: Some(&friend_list)
            },
            friend_list = Box {
                set_vexpand: true,
                set_orientation: Orientation::Vertical,
            },
            search_list = ListBox {
                set_vexpand: true,
                set_css_classes: &["navigation-sidebar"],
                connect_row_activated[sender] => move |_, selected_row| {
                    let index = selected_row.index();
                    sender.input(FriendsMsg::SelectSearchItem(index));
                },
            }
        }

        root.append(&header_bar);
        root.append(&scrolled_window);

        let friend_list_factory: FactoryVecDeque<Box, FriendsGroup, FriendsMsg> =
            FactoryVecDeque::new(friend_list.clone(), &sender.input);
        let search_list_factory: FactoryVecDeque<ListBox, Friend, FriendsMsg> =
            FactoryVecDeque::new(search_list.clone(), &sender.input);

        model.friends_list = Some(friend_list_factory);
        model.search_list = Some(search_list_factory);

        model.render_friends().unwrap();

        ComponentParts {
            model,
            widgets: FriendsWidgets {
                friend_list,
                search_list,
                scrolled_window,
            },
        }
    }

    fn update_with_view(
        &mut self,
        widgets: &mut Self::Widgets,
        msg: Self::Input,
        sender: &ComponentSender<Self>,
    ) {
        use FriendsMsg::*;
        match msg {
            SelectChatroom(account, is_group) => {
                sender.output(ContactMsg::SelectChatroom(account, is_group));
            }
            SelectSearchItem(index) => {
                let account = self.search_list.as_ref().unwrap().get(index as usize).id;
                sender.input(SelectChatroom(account, false));
            }
            Refresh => {
                self.is_refresh_button_enabled = false;
                task::spawn(refresh_friends(sender.clone()));
            }
            Render => {
                match self.render_friends() {
                    Ok(_) => sender.output(ContactMsg::PushToast(
                        "Refreshed the friends list.".to_string(),
                    )),
                    Err(err) => sender.output(ContactMsg::PushToast(err.to_string())),
                }
                self.is_refresh_button_enabled = true;
            }
            Search(keyword) => {
                if keyword.is_empty() {
                    widgets
                        .scrolled_window
                        .set_child(Some(&widgets.friend_list));
                } else {
                    if let Err(err) = self.render_search_result(keyword) {
                        sender.output(ContactMsg::PushToast(err.to_string()))
                    }
                    widgets
                        .scrolled_window
                        .set_child(Some(&widgets.search_list));
                }
            }
        }
    }
}
