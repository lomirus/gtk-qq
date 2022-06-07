use relm4::factory::{DynamicIndex, FactoryComponent};
use relm4::gtk::Picture;
use relm4::gtk::gdk_pixbuf::Pixbuf;
use relm4::{adw, gtk, Sender};

use adw::{prelude::*, Avatar};
use gtk::{Align, Box, Label, ListBox, ListBoxRow, Orientation};
use tokio::task;

use crate::db::fs::{get_group_avatar_path, download_group_avatar_file};
use crate::db::sql::Group;

use super::SidebarMsg;

impl FactoryComponent<ListBox, SidebarMsg> for Group {
    type InitParams = Group;
    type Widgets = ();
    type Input = ();
    type Output = ();
    type Command = ();
    type CommandOutput = ();
    type Root = Box;

    fn init_model(
        init_params: Self::InitParams,
        _index: &DynamicIndex,
        _input: &Sender<Self::Input>,
        _output: &Sender<Self::Output>,
    ) -> Self {
        init_params
    }

    fn init_root(&self) -> Self::Root {
        Box::default()
    }

    fn init_widgets(
        &mut self,
        _index: &DynamicIndex,
        root: &Self::Root,
        _returned_widget: &ListBoxRow,
        _input: &Sender<Self::Input>,
        _output: &Sender<Self::Output>,
    ) -> Self::Widgets {
        relm4::view! {
            item = Box {
                set_margin_top: 8,
                set_margin_bottom: 8,
                #[name = "avatar"]
                append = &Avatar {
                    set_text: Some(&self.name),
                    set_show_initials: true,
                    set_size: 48,
                    set_margin_end: 8
                },
                append = &Box {
                    set_orientation: Orientation::Vertical,
                    set_halign: Align::Start,
                    set_spacing: 8,
                    append = &Label {
                        set_xalign: 0.0,
                        set_text: self.name.as_str(),
                        add_css_class: "heading"
                    },
                    append = &Label {
                        set_text: self.id.to_string().as_str(),
                        add_css_class: "caption",
                        set_xalign: 0.0,
                    },
                },
            }
        }

        let avatar_path = get_group_avatar_path(self.id);
        if avatar_path.exists() {
            if let Ok(pixbuf) = Pixbuf::from_file_at_size(avatar_path, 48, 48) {
                let image = Picture::for_pixbuf(&pixbuf);
                if let Some(paintable) = image.paintable() {
                    avatar.set_custom_image(Some(&paintable));
                }
            }
        } else {
            task::spawn(download_group_avatar_file(self.id));
        }

        root.append(&item);
    }
}
