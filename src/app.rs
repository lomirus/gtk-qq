use relm4::{adw, gtk, AppUpdate, Components, Model, RelmComponent, Sender, Widgets};

use adw::prelude::*;
use adw::ApplicationWindow;
use gtk::{Box, Stack, StackTransitionType};

use crate::pages;

pub struct AppModel {
    page: Page,
}

impl AppModel {
    pub fn new() -> Self {
        AppModel { page: Page::Login }
    }
}

enum Page {
    Login,
    Main,
}

pub enum AppMessage {
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
            AppMessage::LoginSuccessful => self.page = Page::Main,
        }
        true
    }
}

impl Model for AppModel {
    type Msg = AppMessage;
    type Widgets = AppWidgets;
    type Components = AppComponents;
}

#[derive(Components)]
pub struct AppComponents {
    login: RelmComponent<pages::login::LoginPageModel, AppModel>,
    main: RelmComponent<pages::main::MainPageModel, AppModel>,
}

#[relm4::widget(pub)]
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
