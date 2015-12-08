use ::widget::Widget;
use ::window::WindowBuilder;


use super::gtk::{
    Window as GtkWindow,
    WindowTrait,
    WindowType
};

impl Widget for GtkWindow {
    type Builder = WindowBuilder;

    fn build(builder: WindowBuilder) -> Self {
        let window = GtkWindow::new(WindowType::Toplevel).unwrap();
        window.set_title(&builder.title_);
        window
    }
}
