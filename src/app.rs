use relm4::Component;
use relm4::{adw, gtk, SimpleComponent, ComponentSender,ComponentController, ComponentParts, Controller};

use adw::prelude::*;
use adw::ApplicationWindow;
use gtk::{Box, Stack, StackTransitionType};

use crate::pages;

pub struct AppModel {
    page: Page,
    login: Controller<pages::login::LoginPageModel>,
    main: Controller<pages::main::MainPageModel>,
}

enum Page {
    Login,
    Main,
}

pub enum AppMessage {
    LoginSuccessful,
}


pub struct AppComponents {
    login: Controller<pages::login::LoginPageModel>,
    main: Controller<pages::main::MainPageModel>,
}

#[relm4::component(pub)]
impl SimpleComponent for AppModel {

    type Widgets = AppWidgets;
    type InitParams = ();
    type Input = AppMessage;
    type Output = ();
    view! {
        main_window = ApplicationWindow {
            set_default_size: args!(960, 540),
            set_content: stack = Some(&Stack) {
                set_transition_type: StackTransitionType::SlideLeft,
                add_child: login_page = &Box {
                    append: model.login.widget(),
                },
                add_child: main_page = &Box {
                    append: model.main.widget(),
                },
            }
        }
    }

    fn update(
        &mut self,
        msg: Self::Input,
        _sender: &ComponentSender<Self>,
    ) {
        match msg {
            AppMessage::LoginSuccessful => self.page = Page::Main,
        }
    }

    fn pre_view() {
        match model.page {
            Page::Login => widgets.stack.set_visible_child(&widgets.login_page),
            Page::Main => widgets.stack.set_visible_child(&widgets.main_page),
        }
    }

    fn init(
        _params: Self::InitParams,
        root: &Self::Root,
        sender: &ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = AppModel {
            page: Page::Login,
            login: pages::login::LoginPageModel::builder().launch(()).forward(&sender.input, |message|{message}),
            main: pages::main::MainPageModel::builder().launch(()).forward(&sender.input, |message|{message}),
        };
        let widgets = view_output!();
        ComponentParts { model, widgets }
    }
}
