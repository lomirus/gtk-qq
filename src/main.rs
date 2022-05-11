mod config;
mod pages;

use relm4::{adw, gtk, AppUpdate, Components, Model, RelmApp, RelmComponent, Sender, Widgets};

use adw::prelude::*;
use adw::ApplicationWindow;
use gtk::{gio, Box, Stack, StackTransitionType};

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
    main: RelmComponent<pages::main::MainPageModel, AppModel>,
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
                add_child: main_page = &Box {
                    append: components.main.root_widget(),
                },
            }
        }
    }

    fn pre_view() {
        match model.page {
            Page::Login => self.stack.set_visible_child(&self.login_page),
            Page::Main => self.stack.set_visible_child(&self.main_page),
        }
    }
}

fn main() {
    let res = gio::Resource::load(
        config::PKGDATA_DIR.to_owned() + "/resources.gresource"
    ).expect("Could not load resources");
    gio::resources_register(&res);

    let application = adw::Application::builder()
        .application_id("indi.lomirus.gtk-qq")
        .build();

    let model = AppModel { page: Page::Login };
    let app = RelmApp::with_app(model, application);
    app.run()
}
