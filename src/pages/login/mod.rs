use std::collections::VecDeque;

use relm4::actions::{RelmAction, RelmActionGroup};
use relm4::{adw, gtk, ComponentParts, ComponentSender, SimpleComponent};

use adw::prelude::*;
use adw::{ActionRow, Avatar, HeaderBar, PreferencesGroup, Toast, ToastOverlay};
use gtk::{Align, Box, Button, Entry, Label, MenuButton, Orientation};

use crate::app::AppMessage;

#[derive(Default, Debug)]
pub struct LoginPageModel {
    account: String,
    password: String,
    toast_stack: VecDeque<String>,
}

#[derive(Debug)]
pub enum LoginPageMsg {
    LoginStart,
    LoginSuccessful,
    AccountChange(String),
    PasswordChange(String),
    PushToast(String),
    ShiftToast,
}

pub struct LoginPageWidgets {
    headerbar: HeaderBar,
    toast_overlay: ToastOverlay,
}

impl SimpleComponent for LoginPageModel {
    type Widgets = LoginPageWidgets;
    type InitParams = ();
    type Input = LoginPageMsg;
    type Output = AppMessage;
    type Root = Box;

    fn update(&mut self, msg: LoginPageMsg, sender: &ComponentSender<Self>) {
        use LoginPageMsg::*;
        match msg {
            LoginStart => {
                println!("{:?}", self);
                if self.account == "" {
                    sender.input(PushToast("Account cannot be empty".to_string()));
                    return;
                }
                if self.password == "" {
                    sender.input(PushToast("Password cannot be empty".to_string()));
                    return;
                }
                sender.input(LoginPageMsg::LoginSuccessful);
            }
            LoginSuccessful => sender.output(AppMessage::LoginSuccessful),
            AccountChange(new_account) => self.account = new_account,
            PasswordChange(new_password) => self.password = new_password,
            PushToast(message) => self.toast_stack.push_back(message),
            ShiftToast => {
                self.toast_stack
                    .pop_front()
                    .expect("failed to pop from toast stack");
            }
        }
    }

    fn init(
        init_params: Self::InitParams,
        root: &Self::Root,
        sender: &ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        relm4::menu! {
            main_menu: {
                "Keyboard Shortcuts" => ShortcutsAction,
                "About Gtk QQ" => AboutAction
            }
        }

        relm4::view! {
            headerbar = &HeaderBar {
                set_title_widget = Some(&Label) {
                    set_label: "Login"
                },
                pack_end = &Button {
                    set_icon_name: "go-next",
                    connect_clicked(sender) => move |_| {
                        sender.input(LoginPageMsg::LoginStart);
                    }
                },
                pack_end = &MenuButton {
                    set_icon_name: "menu-symbolic",
                    set_menu_model: Some(&main_menu),
                }
            }
        }

        relm4::view! {
            toast_overlay = &ToastOverlay {
                set_child = Some(&Box) {
                    set_halign: Align::Center,
                    set_valign: Align::Center,
                    set_vexpand: true,
                    set_spacing: 32,
                    append = &Avatar {
                        set_text: Some("ADW"),
                        set_size: 72,
                    },
                    append = &PreferencesGroup {
                        add = &ActionRow {
                            set_title: "Account",
                            add_suffix = &Entry {
                                set_valign: Align::Center,
                                set_placeholder_text: Some("Please input your QQ account"),
                                connect_changed(sender) => move |e| {
                                    sender.input(LoginPageMsg::AccountChange(e.buffer().text()));
                                }
                            },
                        },
                        add = &ActionRow {
                            set_title: "Password",
                            add_suffix = &Entry {
                                set_valign: Align::Center,
                                set_placeholder_text: Some("Please input your QQ password"),
                                connect_changed(sender) => move |e| {
                                    sender.input(LoginPageMsg::PasswordChange(e.buffer().text()));
                                }
                            },
                        },
                    },
                },
            }
        }

        relm4::new_action_group!(WindowActionGroup, "menu");
        relm4::new_stateless_action!(ShortcutsAction, WindowActionGroup, "shortcuts");
        relm4::new_stateless_action!(AboutAction, WindowActionGroup, "about");

        let shortcuts_action: RelmAction<ShortcutsAction> = RelmAction::new_stateless(move |_| {
            println!("Keyboard Shortcuts");
        });
        let about_action: RelmAction<AboutAction> = RelmAction::new_stateless(move |_| {
            println!("About Gtk QQ");
        });
        let group: RelmActionGroup<WindowActionGroup> = RelmActionGroup::new();
        group.add_action(shortcuts_action);
        group.add_action(about_action);

        let actions = group.into_action_group();
        root.insert_action_group("menu", Some(&actions));

        let model = LoginPageModel::default();

        ComponentParts {
            model,
            widgets: LoginPageWidgets {
                headerbar,
                toast_overlay,
            },
        }
    }

    fn init_root() -> Self::Root {
        relm4::view! {
            login_page = Box {
                set_hexpand: true,
                set_vexpand: true,
                set_orientation: Orientation::Vertical,
            }
        }
        login_page
    }

    fn update_view(&self, widgets: &mut Self::Widgets, sender: &ComponentSender<Self>) {
        if !self.toast_stack.is_empty() {
            let toast_message = self.toast_stack[0].as_str();
            sender.input(LoginPageMsg::ShiftToast);
            widgets.toast_overlay.add_toast(&Toast::new(toast_message));
        }
    }
}
