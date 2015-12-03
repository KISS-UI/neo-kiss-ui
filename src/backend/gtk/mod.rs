use ::window::WindowBuilder;

use super::Backend as AbsBackend;

mod window;

pub struct Backend;


impl AbsBackend for Backend {
    type Window = self::window::Window;

    fn start(builder: WindowBuilder) {
    
    }
}
