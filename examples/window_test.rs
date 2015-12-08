#[macro_use] extern crate kiss_ui;

use kiss_ui::Window;

builder! {
    create_window(Window) { 
        title => "Hello, world!";
    }
}

fn main() {
    kiss_ui::start(create_window);
}
