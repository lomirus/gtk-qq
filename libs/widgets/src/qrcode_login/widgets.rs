use relm4::gtk::{prelude::*, Align, Box, Label, Orientation, Picture};

#[derive(Debug)]
pub struct QrCodeLoginWidgets {
    pub(super) qr_code: Picture,
}

impl QrCodeLoginWidgets {
    pub(super) fn new(root: &Box) -> Self {
        relm4::view! {
            #[name = "wrapper"]
            Box {
                set_halign: Align::Center,
                set_valign: Align::Center,
                set_orientation: Orientation::Vertical,
                #[name = "qr_code"]
                Picture {
                    set_halign: Align::Center,
                    set_valign: Align::Start,
                },
                Label {
                    set_label: "Scan the QrCode to login",
                    set_halign: Align::Center,
                    set_valign: Align::Center,
                    set_margin_top: 18
                }
            }
        }

        root.append(&wrapper);

        Self { qr_code }
    }
}
