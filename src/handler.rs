use async_trait::async_trait;
use ricq::handler::{Handler, QEvent::*};

pub struct AppHandler;

#[async_trait]
impl Handler for AppHandler {
    async fn handle(&self, event: ricq::handler::QEvent) {
        match event {
            Login(_) => {
                println!("Login");
            }
            GroupMessage(_) => {
                println!("GroupMessage");
            }
            GroupAudioMessage(_) => {
                println!("GroupAudioMessage");
            }
            SelfGroupMessage(_) => {
                println!("SelfGroupMessage");
            }
            FriendMessage(_) => {
                println!("FriendMessage");
            }
            FriendAudioMessage(_) => {
                println!("FriendAudioMessage");
            }
            TempMessage(_) => {
                println!("TempMessage");
            }
            GroupRequest(_) => {
                println!("GroupRequest");
            }
            SelfInvited(_) => {
                println!("SelfInvited");
            }
            FriendRequest(_) => {
                println!("FriendRequest");
            }
            NewMember(_) => {
                println!("NewMember");
            }
            GroupMute(_) => {
                println!("GroupMute");
            }
            FriendMessageRecall(_) => {
                println!("FriendMessageRecall");
            }
            GroupMessageRecall(_) => {
                println!("GroupMessageRecall");
            }
            NewFriend(_) => {
                println!("NewFriend");
            }
            GroupLeave(_) => {
                println!("GroupLeave");
            }
            GroupDisband(_) => {
                println!("GroupDisband");
            }
            FriendPoke(_) => {
                println!("FriendPoke");
            }
            GroupNameUpdate(_) => {
                println!("GroupNameUpdate");
            }
            DeleteFriend(_) => {
                println!("DeleteFriend");
            }
            MemberPermissionChange(_) => {
                println!("MemberPermissionChange");
            }
            KickedOffline(_) => {
                println!("KickedOffline");
            }
            MSFOffline(_) => {
                println!("MSFOffline");
            }
        };
    }
}
