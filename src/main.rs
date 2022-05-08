use relm4::{
    adw, gtk, AppUpdate, Components, Model, RelmApp, RelmComponent, Sender,
    Widgets,
};

use adw::{ApplicationWindow, HeaderBar};
use gtk::{Align, Box, Label, Orientation, Stack, StackTransitionType};

use adw::prelude::*;

mod pages;

struct AppModel {
    page: Page,
}

enum Page {
    Login,
    Main,
}

enum Message {
    LoginSuccessful,
}

impl AppUpdate for AppModel {
    fn update(
        &mut self,
        msg: Self::Msg,
        _components: &Self::Components,
        _sender: relm4::Sender<Self::Msg>,
    ) -> bool {
        match msg {
            Message::LoginSuccessful => self.page = Page::Main,
        }
        true
    }
}

impl Model for AppModel {
    type Msg = Message;
    type Widgets = AppWidgets;
    type Components = AppComponents;
}

#[derive(Components)]
struct AppComponents {
    login: RelmComponent<pages::login::LoginPageModel, AppModel>,
}

#[relm4::widget]
impl Widgets<AppModel, ()> for AppWidgets {
    view! {
        main_window = ApplicationWindow {
            set_default_size: args!(960, 540),
            set_content: stack = Some(&Stack) {
                set_transition_type: StackTransitionType::SlideLeft,
                add_child: login_page = &Box {
                    append: components.login.root_widget(),
                },
                add_child: main_panel = &Box {
                    set_orientation: Orientation::Vertical,
                    append = &HeaderBar {
                        set_title_widget = Some(&Label) {
                            set_label: "GTK4 QQ"
                        },
                    },
                    append = &Box {
                        set_halign: Align::Center,
                        set_valign: Align::Center,
                        set_vexpand: true,
                        set_orientation: Orientation::Vertical,
                        append = &Box {
                            append = &Label {
                                set_label: "Hello, World!"
                            },
                        }
                    }
                }

            }
        }
    }

    fn pre_view() {
        match model.page {
            Page::Login => self.stack.set_visible_child(&self.login_page),
            Page::Main => self.stack.set_visible_child(&self.main_panel),
        }
    }
}

fn main() {
    let model = AppModel { page: Page::Login };
    let app = RelmApp::new(model);
    app.run()
}
