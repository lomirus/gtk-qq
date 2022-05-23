use relm4::{
    adw, gtk, Component, ComponentController, ComponentParts, ComponentSender, Controller,
    SimpleComponent,
};

use adw::{prelude::*, ApplicationWindow};
use gtk::{Box, Stack, StackTransitionType};

use crate::{
    actions::create_gactions,
    pages::{login::LoginPageModel, main::MainPageModel},
};

pub struct AppModel {
    page: Page,
    login: Controller<LoginPageModel>,
    main: Controller<MainPageModel>,
}

enum Page {
    Login,
    Main,
}

#[derive(Debug)]
pub enum AppMessage {
    LoginSuccessful,
}

#[relm4::component(pub)]
impl SimpleComponent for AppModel {
    type Widgets = AppWidgets;
    type InitParams = ();
    type Input = AppMessage;
    type Output = ();
    view! {
        ApplicationWindow {
            set_default_size: (960, 540),
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

    fn update(&mut self, msg: Self::Input, _sender: &ComponentSender<Self>) {
        match msg {
            AppMessage::LoginSuccessful => self.page = Page::Main,
        }
    }

    fn pre_view() {
        match model.page {
            Page::Login => stack.set_visible_child(login_page),
            Page::Main => stack.set_visible_child(main_page),
        }
    }

    fn init(
        _params: Self::InitParams,
        root: &Self::Root,
        sender: &ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = AppModel {
            page: Page::Login,
            login: LoginPageModel::builder()
                .launch(())
                .forward(&sender.input, |message| message),
            main: MainPageModel::builder()
                .launch(())
                .forward(&sender.input, |message| message),
        };
        let widgets = view_output!();

        let actions = create_gactions(root.clone());
        root.insert_action_group("menu", Some(&actions));

        ComponentParts { model, widgets }
    }
}
