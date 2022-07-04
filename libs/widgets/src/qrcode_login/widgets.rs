use relm4::gtk::{traits::BoxExt, Align, Box, Label, Picture};

pub struct QrCodeLoginWidgets {
    pub(super) qr_code: Picture,
    _label: Label,
}

impl QrCodeLoginWidgets {
    pub(super) fn new(root: &Box) -> Self {
        let qr_code = Picture::builder()
            .halign(Align::Center)
            .valign(Align::Center)
            .name("Login Qr Code")
            .build();
        let label = Label::builder()
            .label("Scan the QrCode to login")
            .valign(Align::Center)
            .build();

        root.append(&qr_code);
        root.append(&label);

        Self {
            qr_code,
            _label: label,
        }
    }
}
