#[derive(Debug, Clone)]
#[allow(dead_code)]
pub(crate) enum Content {
    Text(String),
    Image { url: String, filename: String },
}

impl Content {
    pub(crate) fn text(&self) -> String {
        match self {
            Content::Text(text) => text.clone(),
            Content::Image { .. } => "[图片]".to_string(),
        }
    }
}

pub(crate) fn get_text_from(contents: &[Content]) -> String {
    contents
        .iter()
        .map(|content| content.text())
        .collect::<Vec<String>>()
        .join("")
}
