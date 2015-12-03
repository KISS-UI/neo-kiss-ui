use ::window::WindowBuilder;
use ::widget::Widget;

#[cfg(target_os = "windows")]
mod win32;

#[cfg(any(target_os = "linux", feature = "gtk"))]
mod gtk;

#[cfg(all(target_os = "windows", not(feature = "gtk")))]
pub type BackendImpl = win32::Backend;

#[cfg(any(target_os = "linux", feature = "gtk"))]
pub type BackendImpl = gtk::Backend;

pub trait Backend {
    type Window: Widget + From<WindowBuilder>;

    fn start(window: WindowBuilder);
}
