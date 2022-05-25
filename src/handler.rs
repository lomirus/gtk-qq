use std::collections::HashMap;
use std::sync::Arc;

use async_trait::async_trait;
use once_cell::sync::OnceCell;
use ricq::client::event::*;
use ricq::handler::{Handler, QEvent::*};
use ricq::structs::{FriendGroupInfo, FriendInfo};
use ricq::Client;

use crate::pages::main::{MainMsg, MAIN_SENDER};

#[derive(Debug)]
pub struct FriendGroup {
    pub id: u8,
    pub name: String,
    pub friends: Vec<FriendInfo>,
}
pub struct AppHandler;

pub static CLIENT: OnceCell<Arc<Client>> = OnceCell::new();
pub static ACCOUNT: OnceCell<i64> = OnceCell::new();
pub static FRIEND_LIST: OnceCell<Vec<FriendInfo>> = OnceCell::new();
pub static FRIEND_GROUP_LIST: OnceCell<Vec<FriendGroup>> = OnceCell::new();

pub fn init_friends_list(
    friends_list: Vec<FriendInfo>,
    friend_groups: HashMap<u8, FriendGroupInfo>,
) {
    let mut friend_groups = friend_groups
        .iter()
        .map(|(_, v)| v.clone())
        .collect::<Vec<FriendGroupInfo>>();
    friend_groups.sort_by(|a, b| a.seq_id.cmp(&b.seq_id));
    let friends_group_list: Vec<FriendGroup> = friend_groups
        .iter()
        .map(
            |FriendGroupInfo {
                 group_name,
                 group_id,
                 ..
             }| FriendGroup {
                id: *group_id,
                name: group_name.to_string(),
                friends: friends_list
                    .iter()
                    .cloned()
                    .filter(|friend| friend.group_id == *group_id)
                    .collect(),
            },
        )
        .collect::<Vec<FriendGroup>>();

    FRIEND_LIST.set(friends_list).unwrap();
    FRIEND_GROUP_LIST.set(friends_group_list).unwrap();
}

#[async_trait]
impl Handler for AppHandler {
    #[allow(unused_variables)]
    async fn handle(&self, event: ricq::handler::QEvent) {
        match event {
            Login(_) => {}
            GroupMessage(GroupMessageEvent { client, message }) => {
                println!("GroupMessage");
            }
            GroupAudioMessage(GroupAudioMessageEvent { client, message }) => {
                println!("GroupAudioMessage");
            }
            SelfGroupMessage(GroupMessageEvent { client, message }) => {
                println!("SelfGroupMessage");
            }
            FriendMessage(FriendMessageEvent { client, message }) => {
                let main_sender = MAIN_SENDER.get().expect("failed to get main sender");
                main_sender.input(MainMsg::UpdateChatItem(message))
            }
            FriendAudioMessage(FriendAudioMessageEvent { client, message }) => {
                println!("FriendAudioMessage");
            }
            TempMessage(TempMessageEvent { client, message }) => {
                println!("TempMessage");
            }
            GroupRequest(GroupRequestEvent { client, request }) => {
                println!("GroupRequest");
            }
            SelfInvited(SelfInvitedEvent { client, request }) => {
                println!("SelfInvited");
            }
            FriendRequest(FriendRequestEvent { client, request }) => {
                println!("FriendRequest");
            }
            NewMember(NewMemberEvent { client, new_member }) => {
                println!("NewMember");
            }
            GroupMute(GroupMuteEvent { client, group_mute }) => {
                println!("GroupMute");
            }
            FriendMessageRecall(FriendMessageRecallEvent { client, recall }) => {
                println!("FriendMessageRecall");
            }
            GroupMessageRecall(GroupMessageRecallEvent { client, recall }) => {
                println!("GroupMessageRecall");
            }
            NewFriend(NewFriendEvent { client, friend }) => {
                println!("NewFriend");
            }
            GroupLeave(GroupLeaveEvent { client, leave }) => {
                println!("GroupLeave");
            }
            GroupDisband(GroupDisbandEvent { client, disband }) => {
                println!("GroupDisband");
            }
            FriendPoke(FriendPokeEvent { client, poke }) => {
                println!("FriendPoke");
            }
            GroupNameUpdate(GroupNameUpdateEvent { client, update }) => {
                println!("GroupNameUpdate");
            }
            DeleteFriend(DeleteFriendEvent { client, delete }) => {
                println!("DeleteFriend");
            }
            MemberPermissionChange(MemberPermissionChangeEvent { client, change }) => {
                println!("MemberPermissionChange");
            }
            KickedOffline(KickedOfflineEvent { client, offline }) => {
                println!("KickedOffline");
            }
            MSFOffline(MSFOfflineEvent { client, offline }) => {
                println!("MSFOffline");
            }
        };
    }
}
