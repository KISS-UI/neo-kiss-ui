use backend::{Backend, BackendImpl};

use widget::Widget;

pub struct Window(<BackendImpl as Backend>::Window);

impl Widget for Window {
    type Builder = WindowBuilder;
}

#[derive(Default)]
pub struct WindowBuilder {
    pub title_: String,
}

impl WindowBuilder {
    pub fn title<S: Into<String>>(self, title: S) -> Self {
        self.title_ = title.into();
        self
    }
}
