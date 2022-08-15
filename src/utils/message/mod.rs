mod content;
mod utils;

pub(crate) use self::content::get_text_from;
pub(crate) use self::content::Content;
pub(crate) use self::utils::get_contents_from;
#[derive(Clone, Debug)]
pub(crate) struct Message {
    pub sender_id: i64,
    pub sender_name: String,
    pub contents: Vec<Content>,
}

impl Message {
    pub(crate) fn text(&self) -> String {
        get_text_from(&self.contents)
    }
}
