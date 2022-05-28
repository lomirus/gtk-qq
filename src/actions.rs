use relm4::actions::{RelmAction, RelmActionGroup};
use relm4::{adw, gtk};

use adw::ApplicationWindow;
use gtk::{gio::SimpleActionGroup, prelude::*, License};

use crate::config::{APPLICATION_ID, VERSION};

relm4::new_action_group!(pub WindowActionGroup, "menu");
relm4::new_stateless_action!(pub ShortcutsAction, WindowActionGroup, "shortcuts");
relm4::new_stateless_action!(pub AboutAction, WindowActionGroup, "about");

fn show_shortcuts() {
    println!("Keyboard Shortcuts");
}

fn show_about(window: &ApplicationWindow) {
    let dialog = gtk::AboutDialog::builder()
        .comments("Unofficial Linux QQ client, based on GTK4 and libadwaita, developed with Rust and Relm4.")
        .icon_name(APPLICATION_ID)
        .transient_for(window)
        .modal(true)
        .program_name("Gtk QQ")
        .version(VERSION)
        .website_label("Github")
        .website("https://github.com/lomirus/gtk-qq")
        .authors(vec!["Lomirus".into()])
        .license_type(License::Agpl30)
        .build();

    dialog.present();
}

pub fn create_gactions(window: ApplicationWindow) -> SimpleActionGroup {
    let shortcuts_action: RelmAction<ShortcutsAction> =
        RelmAction::new_stateless(|_| show_shortcuts());
    let about_action: RelmAction<AboutAction> =
        RelmAction::new_stateless(move |_| show_about(&window.clone()));

    let group: RelmActionGroup<WindowActionGroup> = RelmActionGroup::new();
    group.add_action(shortcuts_action);
    group.add_action(about_action);

    group.into_action_group()
}
