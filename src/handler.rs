use std::sync::Arc;

use async_trait::async_trait;
use once_cell::sync::OnceCell;
use ricq::client::event::*;
use ricq::handler::{Handler, QEvent::*};
use ricq::msg::elem::{FingerGuessing, RQElem};
use ricq::msg::MessageChain;
use ricq::Client;

use crate::app::main::{MainMsg, Message, MAIN_SENDER};
use crate::db::sql::get_friend_remark;
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
    async fn handle(&self, event: ricq::handler::QEvent) {
        match event {
            Login(_) => {}
            GroupMessage(GroupMessageEvent { message, .. }) => {
                let main_sender = MAIN_SENDER.get().expect("failed to get main sender");
                let content = get_text_from(&message.elements);
                main_sender.input(MainMsg::GroupMessage {
                    group_id: message.group_code,
                    message: Message {
                        sender_id: message.from_uin,
                        sender_name: message.group_card,
                        content: content.clone(),
                    },
                });

                // Send notification
                if &message.from_uin != ACCOUNT.get().unwrap() {
                    let app = APP.get().unwrap();
                    app.notify_group_message(message.group_code, &content);
                }
            }
            #[allow(unused_variables)]
            GroupAudioMessage(GroupAudioMessageEvent { client, message }) => {
                println!("GroupAudioMessage");
            }
            FriendMessage(FriendMessageEvent { message, .. }) => {
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
                    },
                });

                // Send notification
                if message.from_uin != *self_account {
                    let app = APP.get().unwrap();
                    app.notify_friend_message(friend_id, &content);
                }
            }
            #[allow(unused_variables)]
            FriendAudioMessage(FriendAudioMessageEvent { client, message }) => {
                println!("FriendAudioMessage");
            }
            #[allow(unused_variables)]
            TempMessage(TempMessageEvent { client, message }) => {
                println!("TempMessage");
            }
            #[allow(unused_variables)]
            GroupRequest(GroupRequestEvent { client, request }) => {
                println!("GroupRequest");
            }
            #[allow(unused_variables)]
            SelfInvited(SelfInvitedEvent { client, request }) => {
                println!("SelfInvited");
            }
            #[allow(unused_variables)]
            FriendRequest(FriendRequestEvent { client, request }) => {
                println!("FriendRequest");
            }
            #[allow(unused_variables)]
            NewMember(NewMemberEvent { client, new_member }) => {
                println!("NewMember");
            }
            #[allow(unused_variables)]
            GroupMute(GroupMuteEvent { client, group_mute }) => {
                println!("GroupMute");
            }
            #[allow(unused_variables)]
            FriendMessageRecall(FriendMessageRecallEvent { client, recall }) => {
                println!("FriendMessageRecall");
            }
            #[allow(unused_variables)]
            GroupMessageRecall(GroupMessageRecallEvent { client, recall }) => {
                println!("GroupMessageRecall");
            }
            #[allow(unused_variables)]
            NewFriend(NewFriendEvent { client, friend }) => {
                println!("NewFriend");
            }
            #[allow(unused_variables)]
            GroupLeave(GroupLeaveEvent { client, leave }) => {
                println!("GroupLeave");
            }
            #[allow(unused_variables)]
            GroupDisband(GroupDisbandEvent { client, disband }) => {
                println!("GroupDisband");
            }
            #[allow(unused_variables)]
            FriendPoke(FriendPokeEvent { client, poke }) => {
                println!("FriendPoke");
            }
            #[allow(unused_variables)]
            GroupNameUpdate(GroupNameUpdateEvent { client, update }) => {
                println!("GroupNameUpdate");
            }
            #[allow(unused_variables)]
            DeleteFriend(DeleteFriendEvent { client, delete }) => {
                println!("DeleteFriend");
            }
            #[allow(unused_variables)]
            MemberPermissionChange(MemberPermissionChangeEvent { client, change }) => {
                println!("MemberPermissionChange");
            }
            #[allow(unused_variables)]
            KickedOffline(KickedOfflineEvent { client, offline }) => {
                println!("KickedOffline");
            }
            #[allow(unused_variables)]
            MSFOffline(MSFOfflineEvent { client, offline }) => {
                println!("MSFOffline");
            }
        };
    }
}
