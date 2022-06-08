mod builder;
mod widget_impl;

#[derive(Debug)]
pub struct LinkerCopier(Box);
pub use builder::Builder as LinkerCopierBuilder;
use relm4::gtk::Box;

#[allow(dead_code)]
impl LinkerCopier {
    pub fn builder() -> LinkerCopierBuilder {
        LinkerCopierBuilder::new()
    }
}
