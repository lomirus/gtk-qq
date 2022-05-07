use relm4::{
    adw, gtk, send, AppUpdate, ComponentUpdate, Components, Model, RelmApp, RelmComponent, Sender,
    Widgets,
};

use adw::{ApplicationWindow, HeaderBar};
use gtk::{Align, Box, Button, Entry, Grid, Label, Orientation, Stack, StackTransitionType};

use adw::prelude::*;

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

struct LoginPageModel;

impl Model for LoginPageModel {
    type Msg = LoginPageMsg;
    type Widgets = LoginPageWidgets;
    type Components = ();
}

enum LoginPageMsg {
    LoginSuccessful,
}

impl ComponentUpdate<AppModel> for LoginPageModel {
    fn init_model(_parent_model: &AppModel) -> Self {
        LoginPageModel
    }
    fn update(
        &mut self,
        msg: LoginPageMsg,
        _components: &(),
        _sender: Sender<LoginPageMsg>,
        parent_sender: Sender<Message>,
    ) {
        match msg {
            LoginPageMsg::LoginSuccessful => send!(parent_sender, Message::LoginSuccessful),
        }
    }
}

#[relm4::widget]
impl Widgets<LoginPageModel, AppModel> for LoginPageWidgets {
    view! {
        &Box {
            set_hexpand: true,
            set_vexpand: true,
            set_orientation: Orientation::Vertical,
            append = &HeaderBar {
                set_title_widget = Some(&Label) {
                    set_label: "Login"
                },
                pack_end = &Button {
                    set_icon_name: "go-next",
                    connect_clicked(sender) => move |_| {
                        send!(sender, LoginPageMsg::LoginSuccessful);
                    },
                },
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

#[derive(Components)]
struct AppComponents {
    login: RelmComponent<LoginPageModel, AppModel>,
}

#[relm4::widget]
impl Widgets<AppModel, ()> for AppWidgets {
    view! {
        main_window = ApplicationWindow {
            set_content: stack = Some(&Stack) {
                set_transition_type: StackTransitionType::SlideLeft,
                add_child: login_page = &Box {
                    set_hexpand: true,
                    set_vexpand: true,
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
