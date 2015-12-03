pub mod backend;

pub mod widget;
pub mod window;

use backend::Backend;
use window::WindowBuilder;

pub fn start<F, B = backend::BackendImpl>(init_fn: F) where B: Backend,
    F: FnOnce() -> WindowBuilder {
    
    B::init(init_fn());
    B::start_event_loop();
}

macro_rules! builder {
    (($builder_name:ident: $ty:ident) $($field:ident = $expr:expr)*;) => (
        fn $builder_name() -> <$builder_name as ::widget::Widget>::Builder {
            let builder = <$builder_name as ::widget::Widget>::Builder::default();

            $(builder = builder.$field($expr);),*

            builder
        }
    )
}
