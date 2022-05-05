use gtk::prelude::*;
use adw::prelude::*;

use relm4::{adw, gtk, AppUpdate, Model, RelmApp, Widgets};

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
        main_window = adw::ApplicationWindow {
            set_content: main_box = Some(&gtk::Box) {
                set_orientation: gtk::Orientation::Vertical,
                append = &adw::HeaderBar {
                    set_title_widget = Some(&gtk::Label) {
                        set_label: "GTK4 QQ"
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
