use relm4::gtk::{
    builders::{BoxBuilder, ButtonBuilder, LinkButtonBuilder},
    traits::{BoxExt, ButtonExt, WidgetExt},
    Align, Box, Button, LinkButton, Orientation,
};

pub struct Builder {
    state: SetStatus,
    box_w: BoxBuilder,
    link_w: LinkButtonBuilder,
    button_w: ButtonBuilder,
}

impl Builder {
    pub(super) fn new() -> Self {
        Self {
            box_w: Box::builder()
                .orientation(Orientation::Horizontal)
                .halign(Align::Center)
                .valign(Align::Center)
                .spacing(4),
            link_w: LinkButton::builder(),
            button_w: Button::builder().label("copy link"),
            state: SetStatus::None,
        }
    }
    #[allow(dead_code)]
    pub fn text(mut self, text: &str) -> Self {
        self.link_w = self.link_w.label(text);
        self.state = match self.state {
            SetStatus::UrISet => SetStatus::Both,
            SetStatus::None => SetStatus::LabelSet,
            s => s,
        };
        self
    }
    #[allow(dead_code)]
    pub fn url(mut self, url: &str) -> Self {
        self.link_w = self.link_w.uri(url);
        if let SetStatus::None | SetStatus::UrISet = self.state {
            self.link_w = self.link_w.label(url);
        }

        self.state = match self.state {
            SetStatus::LabelSet => SetStatus::Both,
            SetStatus::None => SetStatus::UrISet,
            s => s,
        };
        self
    }
    #[allow(dead_code)]
    pub fn build(self) -> Box {
        let bx = self.box_w.build();

        let link = self.link_w.build();

        let button = self.button_w.build();

        let btn_uri = link.uri().to_string();
        button.connect_clicked(move |e| {
            let clipboard = e.clipboard();
            clipboard.set_text(&btn_uri)
        });

        bx.append(&link);
        bx.append(&button);
        bx
    }
}

enum SetStatus {
    UrISet,
    LabelSet,
    Both,
    None,
}
