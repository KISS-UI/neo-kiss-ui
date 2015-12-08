extern crate gtk;

use ::widget::Widget;
use ::window::WindowBuilder;

use super::Backend as AbsBackend;

use self::gtk::{
    WidgetSignals,
    WidgetTrait,
    Window as GtkWindow
};

mod window;

pub struct Backend;

impl AbsBackend for Backend {
    type Window = GtkWindow;

    fn start(builder: WindowBuilder) {
        self::gtk::init().expect("Error initializing GTK+");

        {
            let main_window = <GtkWindow as Widget>::build(builder);
            main_window.show_all();
            main_window.connect_delete_event(|_, _| {
                self::gtk::main_quit();
                self::gtk::signal::Inhibit(false)
            });
        }

        self::gtk::main();        
    }
}
