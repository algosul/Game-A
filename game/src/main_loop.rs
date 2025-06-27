use winit::platform::run_on_demand::EventLoopExtRunOnDemand;

use crate::App;
pub trait MainLoop {
    async fn run(self);
}
#[derive(Debug)]
pub struct WinitMainLoop;
impl MainLoop for WinitMainLoop {
    async fn run(self) {
        let mut event_loop = winit::event_loop::EventLoop::builder().build().unwrap();
        let mut app = App::new();
        event_loop.run_app_on_demand(&mut app).unwrap()
    }
}
