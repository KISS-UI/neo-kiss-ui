pub mod backend;

pub mod widget;
pub mod window;

use backend::{Backend, BackendImpl};
use window::WindowBuilder;

pub use window::Window;

pub fn start<F>(init_fn: F) where F: FnOnce() -> WindowBuilder {
    
    BackendImpl::start(init_fn());
}

#[macro_export]
macro_rules! builder {
    ($builder_name:ident($ty:ident) { $($bfield:ident => $expr:expr);*; }) => (
        fn $builder_name() -> <$ty as $crate::widget::Widget>::Builder {
            let mut builder = <<$ty as $crate::widget::Widget>::Builder as Default>::default();

            $(builder = builder.$bfield($expr);),*

            builder
        }
    )
}
