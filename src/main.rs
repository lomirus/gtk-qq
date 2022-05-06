use relm4::{adw, gtk, AppUpdate, Model, RelmApp, Widgets};

use adw::{ApplicationWindow, HeaderBar};
use gtk::{Align, Box, Entry, Grid, Label, Orientation, Button};

use adw::prelude::*;

#[derive(Default)]
struct AppModel;

enum Message {}

impl AppUpdate for AppModel {
    fn update(
        &mut self,
        _msg: Self::Msg,
        _components: &Self::Components,
        _sender: relm4::Sender<Self::Msg>,
    ) -> bool {
        true
    }
}

impl Model for AppModel {
    type Msg = Message;
    type Widgets = AppWidgets;
    type Components = ();
}

#[relm4::widget]
impl Widgets<AppModel, ()> for AppWidgets {
    view! {
        main_window = ApplicationWindow {
            set_content: main_box = Some(&Box) {
                set_orientation: Orientation::Vertical,
                append = &HeaderBar {
                    set_title_widget = Some(&Label) {
                        set_label: "GTK4 QQ"
                    },
                    pack_end: &Button::from_icon_name("go-next"),
                    pack_end: &Button::from_icon_name("dialog-information"),
                },
                append = &Box {
                    set_halign: Align::Center,
                    set_valign: Align::Center,
                    set_vexpand: true,
                    set_orientation: Orientation::Vertical,
                    append = &Box {
                        append = &Grid {
                            set_row_spacing: 12,
                            set_column_spacing: 12,
                            attach(0, 0, 1, 1) = &Label {
                                set_label: "Account"
                            },
                            attach(1, 0, 1, 1) = &Entry {
                                set_placeholder_text: Some("请输入您的QQ号码")
                            },
                            attach(0, 1, 1, 1) = &Label {
                                set_label: "Password"
                            },
                            attach(1, 1, 1, 1) = &Entry {
                                set_placeholder_text: Some("请输入您的QQ密码")
                            }
                        }
                    }
                }
            }
        }
    }
}

fn main() {
    let model = AppModel::default();
    let app = RelmApp::new(model);
    app.run()
}
