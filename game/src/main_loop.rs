use winit::platform::run_on_demand::EventLoopExtRunOnDemand;

use crate::{render::Surface, App};
pub trait MainLoop {
    fn run<R: Surface>(self);
}
#[derive(Debug)]
pub struct WinitMainLoop;
impl MainLoop for WinitMainLoop {
    fn run<R: Surface>(self) {
        let mut event_loop = winit::event_loop::EventLoop::builder().build().unwrap();
        let mut app = App::<R>::new();
        event_loop.run_app_on_demand(&mut app).unwrap()
    }
}
