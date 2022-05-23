use async_trait::async_trait;
use ricq::client::event::*;
use ricq::handler::{Handler, QEvent::*};

pub struct AppHandler;

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
                println!("FriendMessage");
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
