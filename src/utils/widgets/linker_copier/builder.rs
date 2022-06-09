use relm4::gtk::{
    traits::{BoxExt, ButtonExt, WidgetExt},
    Align, Box, Button, LinkButton, Orientation,
};

pub struct Builder {
    text: Option<String>,
    url: Option<String>,
}

impl Builder {
    pub(super) fn new() -> Self {
        Self {
            text: None,
            url: None,
        }
    }
    #[allow(dead_code)]
    pub fn text(mut self, text: &str) -> Self {
        self.text.replace(text.to_string());
        self
    }
    #[allow(dead_code)]
    pub fn url(mut self, url: &str) -> Self {
        self.url.replace(url.to_string());
        self
    }
    #[allow(dead_code)]
    pub fn build(self) -> Box {
        let bx = Box::builder()
            .orientation(Orientation::Horizontal)
            .halign(Align::Center)
            .valign(Align::Center)
            .spacing(4)
            .build();

        let uri = self.url.expect("LinkerCopier Need provide Uri");

        let button = Button::builder().label("copy link").build();

        let btn_uri = uri.clone();
        button.connect_clicked(move |e| {
            let clipboard = e.clipboard();
            clipboard.set_text(&btn_uri)
        });

        let link = LinkButton::builder()
            .uri(&uri)
            .label(&self.text.unwrap_or(uri))
            .build();

        bx.append(&link);
        bx.append(&button);
        bx
    }
}
