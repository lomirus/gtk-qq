use std::sync::Arc;

use async_trait::async_trait;
use once_cell::sync::OnceCell;
use ricq::client::event::*;
use ricq::handler::{Handler, QEvent::*};
use ricq::msg::elem::{FingerGuessing, RQElem};
use ricq::msg::MessageChain;
use ricq::Client;

use crate::app::main::{MainMsg, MAIN_SENDER, Message};
use crate::db::{get_friend_remark, get_group_name};
use crate::APP;

pub struct AppHandler;

pub static CLIENT: OnceCell<Arc<Client>> = OnceCell::new();
pub static ACCOUNT: OnceCell<i64> = OnceCell::new();

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
                let content = get_text_from(&message.elements);
                main_sender.input(MainMsg::GroupMessage {
                    group_id: message.group_code,
                    message: Message {
                        sender_id: message.from_uin,
                        sender_name: message.group_card,
                        content: content.clone(),
                    }
                });

                // Send notification
                let app = APP.get().unwrap();
                let group_name: String = get_group_name(message.group_code);
                app.send_notification(&group_name, &content);
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
                let content = get_text_from(&message.elements);
                main_sender.input(MainMsg::FriendMessage {
                    friend_id,
                    message: Message {
                        sender_id: message.from_uin,
                        sender_name: get_friend_remark(message.from_uin),
                        content: content.clone(),
                    }
                });

                // Send notification
                let app = APP.get().unwrap();
                app.send_notification(&get_friend_remark(friend_id), &content);
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
