use std::sync::Arc;

use async_trait::async_trait;
use once_cell::sync::OnceCell;
use ricq::client::event::*;
use ricq::handler::{Handler, QEvent::*};
use ricq::Client;

use crate::app::main::{MainMsg, MAIN_SENDER};
use crate::db::sql::get_friend_remark;
use crate::utils::message::{get_contents_from, get_text_from, Message};
use crate::APP;

pub struct AppHandler;

pub static CLIENT: OnceCell<Arc<Client>> = OnceCell::new();
pub static ACCOUNT: OnceCell<i64> = OnceCell::new();

#[async_trait]
impl Handler for AppHandler {
    async fn handle(&self, event: ricq::handler::QEvent) {
        match event {
            Login(_) => {}
            GroupMessage(GroupMessageEvent { inner, .. }) => {
                let main_sender = MAIN_SENDER.get().expect("failed to get main sender");
                let content = get_contents_from(&inner.elements);
                main_sender.input(MainMsg::GroupMessage {
                    group_id: inner.group_code,
                    message: Message {
                        sender_id: inner.from_uin,
                        sender_name: inner.group_card,
                        contents: content.clone(),
                    },
                });

                // Send notification
                if &inner.from_uin != ACCOUNT.get().unwrap() {
                    let app = APP.get().unwrap();
                    app.notify_group_message(inner.group_code, &get_text_from(&content));
                }
            }
            #[allow(unused_variables)]
            GroupAudioMessage(GroupAudioMessageEvent { client, inner }) => {
                println!("GroupAudioMessage");
            }
            FriendMessage(FriendMessageEvent { inner, .. }) => {
                let main_sender = MAIN_SENDER.get().expect("failed to get main sender");
                let self_account = ACCOUNT.get().unwrap();
                let friend_id = if inner.from_uin == *self_account {
                    inner.target
                } else {
                    inner.from_uin
                };
                let contents = get_contents_from(&inner.elements);
                main_sender.input(MainMsg::FriendMessage {
                    friend_id,
                    message: Message {
                        sender_id: inner.from_uin,
                        sender_name: get_friend_remark(inner.from_uin),
                        contents: contents.clone(),
                    },
                });

                // Send notification
                if inner.from_uin != *self_account {
                    let app = APP.get().unwrap();
                    app.notify_friend_message(friend_id, &get_text_from(&contents));
                }
            }
            #[allow(unused_variables)]
            FriendAudioMessage(FriendAudioMessageEvent { client, inner }) => {
                println!("FriendAudioMessage");
            }
            #[allow(unused_variables)]
            GroupTempMessage(GroupTempMessageEvent { client, inner }) => {
                println!("GroupTempMessage");
            }
            #[allow(unused_variables)]
            SelfInvited(SelfInvitedEvent { client, inner }) => {
                println!("SelfInvited");
            }
            #[allow(unused_variables)]
            NewMember(NewMemberEvent { client, inner }) => {
                println!("NewMember");
            }
            #[allow(unused_variables)]
            GroupMute(GroupMuteEvent { client, inner }) => {
                println!("GroupMute");
            }
            #[allow(unused_variables)]
            FriendMessageRecall(FriendMessageRecallEvent { client, inner }) => {
                println!("FriendMessageRecall");
            }
            #[allow(unused_variables)]
            GroupMessageRecall(GroupMessageRecallEvent { client, inner }) => {
                println!("GroupMessageRecall");
            }
            #[allow(unused_variables)]
            NewFriend(NewFriendEvent { client, inner }) => {
                println!("NewFriend");
            }
            #[allow(unused_variables)]
            GroupLeave(GroupLeaveEvent { client, inner }) => {
                println!("GroupLeave");
            }
            #[allow(unused_variables)]
            GroupDisband(GroupDisbandEvent { client, inner }) => {
                println!("GroupDisband");
            }
            #[allow(unused_variables)]
            FriendPoke(FriendPokeEvent { client, inner }) => {
                println!("FriendPoke");
            }
            #[allow(unused_variables)]
            GroupNameUpdate(GroupNameUpdateEvent { client, inner }) => {
                println!("GroupNameUpdate");
            }
            #[allow(unused_variables)]
            DeleteFriend(DeleteFriendEvent { client, inner }) => {
                println!("DeleteFriend");
            }
            #[allow(unused_variables)]
            MemberPermissionChange(MemberPermissionChangeEvent { client, inner }) => {
                println!("MemberPermissionChange");
            }
            #[allow(unused_variables)]
            KickedOffline(KickedOfflineEvent { client, inner }) => {
                println!("KickedOffline");
            }
            #[allow(unused_variables)]
            MSFOffline(MSFOfflineEvent { client, inner }) => {
                println!("MSFOffline");
            }
            #[allow(unused_variables)]
            GroupRequest(_) => {
                println!("GroupRequest");
            }
            #[allow(unused_variables)]
            NewFriendRequest(_) => {
                println!("NewFriendRequest");
            }
        };
    }
}
