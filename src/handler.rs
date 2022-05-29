use std::collections::HashMap;
use std::sync::Arc;

use async_trait::async_trait;
use once_cell::sync::OnceCell;
use ricq::client::event::*;
use ricq::handler::{Handler, QEvent::*};
use ricq::msg::elem::{FingerGuessing, RQElem};
use ricq::msg::MessageChain;
use ricq::structs::{FriendGroupInfo, FriendInfo, GroupInfo};
use ricq::Client;

use crate::app::main::ContactGroup;
use crate::app::main::{MainMsg, MAIN_SENDER};

pub struct AppHandler;

pub static CLIENT: OnceCell<Arc<Client>> = OnceCell::new();
pub static ACCOUNT: OnceCell<i64> = OnceCell::new();
pub static FRIEND_LIST: OnceCell<Vec<FriendInfo>> = OnceCell::new();
pub static FRIEND_GROUP_LIST: OnceCell<Vec<ContactGroup>> = OnceCell::new();
pub static GROUP_LIST: OnceCell<Vec<GroupInfo>> = OnceCell::new();

pub fn init_friends_list(
    friends_list: Vec<FriendInfo>,
    friend_groups: HashMap<u8, FriendGroupInfo>,
) {
    let mut friend_groups = friend_groups
        .iter()
        .map(|(_, v)| v.clone())
        .collect::<Vec<FriendGroupInfo>>();
    friend_groups.sort_by(|a, b| a.seq_id.cmp(&b.seq_id));
    let friends_group_list: Vec<ContactGroup> = friend_groups
        .iter()
        .map(
            |FriendGroupInfo {
                 group_name,
                 group_id,
                 ..
             }| ContactGroup {
                id: *group_id,
                name: group_name.to_string(),
                friends: friends_list
                    .iter()
                    .cloned()
                    .filter(|friend| friend.group_id == *group_id)
                    .collect(),
            },
        )
        .collect::<Vec<ContactGroup>>();

    FRIEND_LIST.set(friends_list).unwrap();
    FRIEND_GROUP_LIST.set(friends_group_list).unwrap();
}

fn get_text_from(message_chain: &MessageChain) -> String {
    let mut content = Vec::<String>::new();
    for elem in message_chain.clone() {
        match elem {
            RQElem::At(at) => {
                content.push(format!("[{}({})]", at.display, at.target));
            }
            RQElem::Text(ref text) => {
                content.push(text.content.clone());
            }
            RQElem::Face(face) => {
                content.push(format!("[{}]", face.name));
            }
            RQElem::MarketFace(face) => {
                content.push(format!("[{}]", face.name));
            }
            RQElem::Dice(dice) => {
                content.push(format!("[🎲({})]", dice.value));
            }
            RQElem::FingerGuessing(finger_guessing) => {
                content.push(
                    match finger_guessing {
                        FingerGuessing::Rock => "[✊]",
                        FingerGuessing::Scissors => "[✌]",
                        FingerGuessing::Paper => "[✋]",
                    }
                    .to_string(),
                );
            }
            RQElem::LightApp(light_app) => {
                content.push(format!("[{:#?}]", light_app.content));
            }
            RQElem::RichMsg(rich_msg) => {
                content.push("[RICH MESSAGE]".to_string());
                println!("RichMsg: {:#?}", rich_msg);
            }
            RQElem::FriendImage(_) => {
                content.push("[图片]".to_string());
            }
            RQElem::GroupImage(_) => {
                content.push("[图片]".to_string());
            }
            RQElem::FlashImage(_) => {
                content.push("[闪照]".to_string());
            }
            RQElem::VideoFile(_) => {
                content.push("[视频文件]".to_string());
            }
            RQElem::Other(_) => {}
        }
    }
    content.join(" ")
}

#[async_trait]
impl Handler for AppHandler {
    #[allow(unused_variables)]
    async fn handle(&self, event: ricq::handler::QEvent) {
        match event {
            Login(_) => {}
            GroupMessage(GroupMessageEvent { client, message }) => {
                let main_sender = MAIN_SENDER.get().expect("failed to get main sender");
                main_sender.input(MainMsg::GroupMessage {
                    group_id: message.group_code,
                    sender_id: message.from_uin,
                    content: get_text_from(&message.elements),
                });
            }
            GroupAudioMessage(GroupAudioMessageEvent { client, message }) => {
                println!("GroupAudioMessage");
            }
            FriendMessage(FriendMessageEvent { client, message }) => {
                let main_sender = MAIN_SENDER.get().expect("failed to get main sender");
                let self_account = ACCOUNT.get().unwrap();
                let friend_id = if message.from_uin == *self_account {
                    message.target
                } else {
                    message.from_uin
                };

                main_sender.input(MainMsg::FriendMessage {
                    friend_id,
                    sender_id: message.from_uin,
                    content: get_text_from(&message.elements),
                });
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
