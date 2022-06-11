use std::sync::Arc;

use relm4::{
    adw::{HeaderBar, Window},
    gtk::{
        prelude::EntryBufferExtManual,
        traits::{BoxExt, ButtonExt, EntryExt, GtkWindowExt},
        Align, Box, Button, Entry, Label, Orientation, Picture,
    },
    ComponentSender,
};

use super::{payloads::Output, CaptchaModel};

pub struct CaptchaWidgets {
    _header_bar: HeaderBar,
    _body: Box,
    _body_left_info: Box,
    _body_scan_info_1: Label,
    _body_scan_info_2: Label,
    _ticket_input_area: Box,
    _ticket_label: Label,
    _ticket_input: Entry,
    _ticket_submit_btn: Button,
    _tip_box_1: Box,
    _tip_1: Label,
    _tip_2: Label,
    _tip_3: Label,
    _tip_4: Label,

    _body_right_qrcode: Picture,
}

impl CaptchaWidgets {
    pub fn new(
        root: &Box,
        scanner_link: &Box,
        verify_link: &Box,
        window_ref: &Window,
        sender_ref: &ComponentSender<CaptchaModel>,
    ) -> Self {
        let header_bar = HeaderBar::builder()
            .title_widget(
                &Label::builder()
                    .label("Captcha Verify Introduction")
                    .build(),
            )
            .build();

        let body = Box::builder()
            .valign(Align::Center)
            .halign(Align::Center)
            .vexpand(true)
            .spacing(24)
            .build();

        let body_left_info = Box::builder()
            .margin_bottom(16)
            .margin_top(16)
            .margin_start(16)
            .margin_end(16)
            .orientation(Orientation::Vertical)
            .halign(Align::Start)
            .spacing(8)
            .build();

        let body_scan_info_1 = Label::builder()
            .xalign(0.0)
            .label("1. Install the tool on your Android phone: ")
            .build();

        let body_scan_info_2 = Label::builder()
            .xalign(0.0)
            .label("2. Scan the qrcode and get the ticket.")
            .build();

        let ticket_input_area = Box::new(Orientation::Horizontal, 0);

        let ticket_label = Label::new("3. ".into());

        let ticket_input = Entry::builder()
            .placeholder_text("Paste the ticket here...")
            .margin_end(8)
            .activates_default(true)
            .build();

        let sender = Arc::clone(sender_ref);
        let window = window_ref.clone();
        ticket_input.connect_activate(move |entry| {
            sender.output(Output::Submit {
                ticket: entry.buffer().text(),
            });
            window.close();
        });

        let entry_buf = ticket_input.buffer();

        let ticket_submit_btn = Button::builder().label("Submit Ticket").build();

        let sender = Arc::clone(sender_ref);
        let window = window_ref.clone();
        ticket_submit_btn.connect_clicked(move |_| {
            sender.output(Output::Submit {
                ticket: entry_buf.text(),
            });
            window.close();
        });

        let tip_box = Box::new(Orientation::Horizontal, 0);
        let tip_1 = Label::builder()
            .xalign(0.0)
            .label("Help: If you do not have an Android phone to install the tool, open the")
            .build();

        let tip_2 = Label::builder().xalign(0.0).label(" in the browser manually, open the devtools and switch to the network panel. After you passed the").build();

        let tip_3 = Label::builder().xalign(0.0).label("verification, you will find a request whose response contains the `ticket`. Then just paste it").build();

        let tip_4 = Label::builder().xalign(0.0).label("above. The result would be same. It just maybe more complex if you don't know devtools well.").build();

        let body_right_qrcode = Picture::builder()
            .width_request(240)
            .can_shrink(true)
            .build();
        let mut path = dirs::home_dir().unwrap();
        path.push(".gtk-qq");
        path.push("captcha_url.png");

        body_right_qrcode.set_filename(Some(&path));

        root.append(&header_bar);
        root.append(&body);

        body.append(&body_left_info);

        body_left_info.append(&body_scan_info_1);
        body_left_info.append(scanner_link);
        body_left_info.append(&body_scan_info_2);
        body_left_info.append(&ticket_input_area);

        ticket_input_area.append(&ticket_label);
        ticket_input_area.append(&ticket_input);
        ticket_input_area.append(&ticket_submit_btn);

        body_left_info.append(&tip_box);

        tip_box.append(&tip_1);
        tip_box.append(verify_link);
        body_left_info.append(&tip_2);
        body_left_info.append(&tip_3);
        body_left_info.append(&tip_4);

        body.append(&body_right_qrcode);

        Self {
            _header_bar: header_bar,
            _body: body,
            _body_left_info: body_left_info,
            _body_scan_info_1: body_scan_info_1,
            _body_scan_info_2: body_scan_info_2,
            _ticket_input_area: ticket_input_area,
            _ticket_label: ticket_label,
            _ticket_input: ticket_input,
            _ticket_submit_btn: ticket_submit_btn,
            _tip_1: tip_1,
            _tip_2: tip_2,
            _tip_3: tip_3,
            _tip_4: tip_4,
            _body_right_qrcode: body_right_qrcode,
            _tip_box_1: tip_box,
        }
    }
}
