mod chat_item;
mod friends_group;
mod group_item;

use relm4::factory::FactoryVecDeque;
use relm4::{adw, gtk, ComponentParts, ComponentSender, SimpleComponent, WidgetPlus};
use std::cell::RefCell;

use adw::{prelude::*, HeaderBar, ViewStack, ViewSwitcherBar, ViewSwitcherTitle};
use gtk::{Box, Button, Entry, EntryIconPosition, ListBox, Orientation, ScrolledWindow};
use tokio::task;

use crate::db::{get_db, refresh_friends_list, refresh_groups_list, Friend, Group};

use super::MainMsg;
use chat_item::ChatItem;

pub use self::friends_group::FriendsGroup;

#[derive(Debug)]
pub struct SidebarModel {
    chats_list: Option<RefCell<FactoryVecDeque<ListBox, ChatItem, SidebarMsg>>>,
    friends_list: Option<RefCell<FactoryVecDeque<Box, FriendsGroup, SidebarMsg>>>,
    groups_list: Option<RefCell<FactoryVecDeque<ListBox, Group, SidebarMsg>>>,
    is_refresh_friends_button_enabled: bool,
    is_refresh_groups_button_enabled: bool,
}

impl SidebarModel {
    fn update_chat_item(&self, account: i64, is_group: bool, last_message: String) {
        let mut chats_list = self.chats_list.as_ref().unwrap().borrow_mut();
        for i in 0..chats_list.len() {
            let this_account = chats_list.get(i).account;
            let is_this_group = chats_list.get(i).is_group;
            if this_account == account && is_this_group == is_group {
                chats_list.swap(0, i);
                chats_list.front_mut().unwrap().last_message = last_message;
                break;
            }
        }
    }

    fn insert_chat_item(&self, account: i64, is_group: bool, last_message: String) {
        let mut chats_list = self.chats_list.as_ref().unwrap().borrow_mut();
        chats_list.push_front((account, is_group, last_message));
    }

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
            .prepare("Select id, name from friends_groups")?
            .query_map([], |row| {
                Ok(FriendsGroup {
                    id: row.get(0)?,
                    name: row.get(1)?,
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

async fn refresh_friends(sender: ComponentSender<SidebarModel>) {
    sender.output(MainMsg::PushToast(
        "Start refreshing the friends list...".to_string(),
    ));
    match refresh_friends_list().await {
        Ok(_) => sender.input(SidebarMsg::RenderFriends),
        Err(err) => sender.output(MainMsg::PushToast(err.to_string())),
    }
}

async fn refresh_groups(sender: ComponentSender<SidebarModel>) {
    sender.output(MainMsg::PushToast(
        "Start refreshing the groups list...".to_string(),
    ));
    match refresh_groups_list().await {
        Ok(_) => sender.input(SidebarMsg::RenderGroups),
        Err(err) => sender.output(MainMsg::PushToast(err.to_string())),
    }
}

#[derive(Debug)]
pub enum SidebarMsg {
    SelectChatroom(i32),
    UpdateChatItem(i64, bool, String),
    InsertChatItem(i64, bool, String),
    RefreshFriends,
    RefreshGroups,
    RenderFriends,
    RenderGroups,
}

#[relm4::component(pub)]
impl SimpleComponent for SidebarModel {
    type Input = SidebarMsg;
    type Output = MainMsg;
    type Widgets = MainPageWidgets;
    type InitParams = ();

    view! {
        #[root]
        sidebar = &Box {
            set_vexpand: true,
            set_width_request: 320,
            set_orientation: Orientation::Vertical,
            append = &HeaderBar {
                set_show_start_title_buttons: false,
                set_show_end_title_buttons: false,
                set_title_widget = Some(&ViewSwitcherTitle) {
                    set_title: "Sidebar",
                    set_stack: Some(&stack)
                }
            },
            append: stack = &ViewStack {
                set_vexpand: true,
            }
        },
        _chats = ScrolledWindow {
            set_child: sidebar_chats = Some(&ListBox) {
                set_css_classes: &["navigation-sidebar"],
                connect_row_activated[sender] => move |_, selected_row| {
                    let index = selected_row.index();
                    sender.input(SidebarMsg::SelectChatroom(index));
                },
            }
        },
        _contact = Box {
            set_orientation: Orientation::Vertical,
            append: contact_stack = &ViewStack {
                set_vexpand: true,
            },
            append = &ViewSwitcherBar {
                set_stack: Some(&contact_stack),
                set_reveal: true
            }
        },
        contact_friends = Box {
            set_orientation: Orientation::Vertical,
            Box {
                set_margin_all: 8,
                Button {
                    #[watch]
                    set_sensitive: model.is_refresh_friends_button_enabled,
                    set_tooltip_text: Some("Refresh friends list"),
                    set_icon_name: "view-refresh-symbolic",
                    set_margin_end: 8,
                    connect_clicked[sender] => move |_| {
                        sender.input(SidebarMsg::RefreshFriends);
                    },
                },
                append: search_friends_entry = &Entry {
                    set_icon_from_icon_name: (EntryIconPosition::Secondary, Some("system-search-symbolic")),
                    set_placeholder_text: Some("Search in friends..."),
                    set_width_request: 320 - 3 * 8 - 32
                },
            },
            ScrolledWindow {
                set_child: contact_friends_list = Some(&Box) {
                    set_vexpand: true,
                    set_orientation: Orientation::Vertical,
                }
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
                        sender.input(SidebarMsg::RefreshGroups);
                    },
                },
                append: search_groups_entry = &Entry {
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
                        sender.output(MainMsg::SelectChatroom(account, true));
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
        let mut model = SidebarModel {
            chats_list: None,
            friends_list: None,
            groups_list: None,
            is_refresh_friends_button_enabled: true,
            is_refresh_groups_button_enabled: true,
        };
        let widgets = view_output!();

        let stack: &ViewStack = &widgets.stack;
        let contact_stack: &ViewStack = &widgets.contact_stack;

        let chats = stack.add_titled(&widgets._chats, None, "Chats");
        let contact = stack.add_titled(&widgets._contact, None, "Contact");
        let friends = contact_stack.add_titled(&widgets.contact_friends, None, "Friends");
        let groups = contact_stack.add_titled(&widgets.contact_groups, None, "Groups");

        chats.set_icon_name(Some("chat-symbolic"));
        contact.set_icon_name(Some("address-book-symbolic"));
        friends.set_icon_name(Some("person2-symbolic"));
        groups.set_icon_name(Some("people-symbolic"));

        let chats_list: FactoryVecDeque<ListBox, ChatItem, SidebarMsg> =
            FactoryVecDeque::new(widgets.sidebar_chats.clone(), &sender.input);
        let friends_list: FactoryVecDeque<Box, FriendsGroup, SidebarMsg> =
            FactoryVecDeque::new(widgets.contact_friends_list.clone(), &sender.input);
        let groups_list: FactoryVecDeque<ListBox, Group, SidebarMsg> =
            FactoryVecDeque::new(widgets.contact_groups_list.clone(), &sender.input);

        model.chats_list = Some(RefCell::new(chats_list));
        model.friends_list = Some(RefCell::new(friends_list));
        model.groups_list = Some(RefCell::new(groups_list));

        model.render_friends().unwrap();
        model.render_groups().unwrap();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: SidebarMsg, sender: &ComponentSender<Self>) {
        use SidebarMsg::*;
        match msg {
            SelectChatroom(index) => {
                let chat_item = self.chats_list.as_ref().unwrap().borrow();
                let chat_item = chat_item.get(index as usize);
                let account = chat_item.account;
                let is_group = chat_item.is_group;
                sender.output(MainMsg::SelectChatroom(account, is_group));
            }
            UpdateChatItem(account, is_group, last_message) => {
                self.update_chat_item(account, is_group, last_message)
            }
            InsertChatItem(account, is_group, last_message) => {
                self.insert_chat_item(account, is_group, last_message)
            }
            RefreshFriends => {
                self.is_refresh_friends_button_enabled = false;
                task::spawn(refresh_friends(sender.clone()));
            }
            RefreshGroups => {
                self.is_refresh_groups_button_enabled = false;
                task::spawn(refresh_groups(sender.clone()));
            }
            RenderFriends => {
                match self.render_friends() {
                    Ok(_) => sender.output(MainMsg::PushToast(
                        "Refreshed the friends list.".to_string(),
                    )),
                    Err(err) => sender.output(MainMsg::PushToast(err.to_string())),
                }
                self.is_refresh_friends_button_enabled = true;
            }
            RenderGroups => {
                match self.render_groups() {
                    Ok(_) => {
                        sender.output(MainMsg::PushToast("Refreshed the groups list.".to_string()))
                    }
                    Err(err) => sender.output(MainMsg::PushToast(err.to_string())),
                }
                self.is_refresh_groups_button_enabled = true;
            }
        }
        self.chats_list.as_ref().unwrap().borrow().render_changes();
    }
}
