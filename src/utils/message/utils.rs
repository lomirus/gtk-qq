use super::Content;
use ricq::msg::elem::{FingerGuessing, FlashImage, RQElem};
use ricq::msg::MessageChain;

pub(crate) fn get_contents_from(message_chain: &MessageChain) -> Vec<Content> {
    let mut contents = Vec::<Content>::new();
    for elem in message_chain.clone() {
        match elem {
            RQElem::At(at) => {
                contents.push(Content::Text(format!("[{}({})]", at.display, at.target)));
            }
            RQElem::Text(ref text) => {
                contents.push(Content::Text(text.content.clone()));
            }
            RQElem::Face(face) => {
                contents.push(Content::Text(format!("[{}]", face.name)));
            }
            RQElem::MarketFace(face) => {
                contents.push(Content::Text(format!("[{}]", face.name)));
            }
            RQElem::Dice(dice) => {
                contents.push(Content::Text(format!("[🎲({})]", dice.value)));
            }
            RQElem::FingerGuessing(finger_guessing) => {
                contents.push(Content::Text(
                    match finger_guessing {
                        FingerGuessing::Rock => "[✊]",
                        FingerGuessing::Scissors => "[✌]",
                        FingerGuessing::Paper => "[✋]",
                    }
                    .to_string(),
                ));
            }
            RQElem::LightApp(light_app) => {
                contents.push(Content::Text("[LIGHT_APP MESSAGE]".to_string()));
                println!("LightApp: {:#?}", light_app);
            }
            RQElem::RichMsg(rich_msg) => {
                contents.push(Content::Text("[RICH MESSAGE]".to_string()));
                println!("RichMsg: {:#?}", rich_msg);
            }
            RQElem::FriendImage(image) => {
                let content = Content::Image {
                    url: image.url(),
                    filename: image.file_path,
                };
                contents.push(content);
            }
            RQElem::GroupImage(image) => {
                let content = Content::Image {
                    url: image.url(),
                    filename: image.file_path,
                };
                contents.push(content);
            }
            RQElem::FlashImage(image) => {
                let content = Content::Image {
                    url: image.url(),
                    filename: get_flash_image_path(image),
                };
                contents.push(content);
            }
            RQElem::VideoFile(_) => {
                contents.push(Content::Text("[视频文件]".to_string()));
            }
            RQElem::Other(_) => {}
        }
    }
    contents
}

fn get_flash_image_path(image: FlashImage) -> String {
    match image {
        FlashImage::FriendImage(image) => image.file_path,
        FlashImage::GroupImage(image) => image.file_path,
    }
}
