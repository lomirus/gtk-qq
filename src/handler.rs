use std::sync::Arc;

use async_trait::async_trait;
use once_cell::sync::OnceCell;
use ricq::client::event::*;
use ricq::handler::{Handler, QEvent::*};
use ricq::structs::FriendInfo;
use ricq::Client;

use crate::pages::main::{MainMsg, MAIN_SENDER};

pub struct AppHandler;

pub static CLIENT: OnceCell<Arc<Client>> = OnceCell::new();
pub static ACCOUNT: OnceCell<i64> = OnceCell::new();
pub static FRIEND_LIST: OnceCell<Vec<FriendInfo>> = OnceCell::new();

#[async_trait]
impl Handler for AppHandler {
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
