use relm4::{gtk, SimpleComponent};

mod payloads;
mod widgets;

pub use payloads::{Output, Payload};

pub struct DeviceLock;

impl SimpleComponent for DeviceLock {
    type Input = ();

    type Output = payloads::Output;

    type InitParams = payloads::Payload;

    type Root = gtk::Box;

    type Widgets = widgets::Widgets;

    fn init_root() -> Self::Root {
        gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .build()
    }

    fn init(
        params: Self::InitParams,
        root: &Self::Root,
        sender: &relm4::ComponentSender<Self>,
    ) -> relm4::ComponentParts<Self> {
        let widgets = widgets::Widgets::new(root, params, sender);

        relm4::ComponentParts {
            model: Self,
            widgets,
        }
    }
}
