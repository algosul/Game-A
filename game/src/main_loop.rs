use winit::platform::run_on_demand::EventLoopExtRunOnDemand;

use crate::{
    render::Surface,
    scene::{scenes::DynamicScene, Scene},
    App,
};
pub trait MainLoop {
    fn game_loop();
    fn run<R: Surface>(self);
}
pub struct WinitMainLoop;
impl MainLoop for WinitMainLoop {
    fn game_loop() {
        let mut scene = Box::new(DynamicScene::new());
        loop {
            for object in scene.get_mut_objects() {
                object.update(0.0);
            }
        }
    }

    // fn render_loop() {
    //     loop {
    //         for object in scene.get_mut_objects() {
    //             object.update(0.0);
    //         }
    //     }
    // }
    fn run<R: Surface>(self) {
        let mut event_loop = winit::event_loop::EventLoop::builder().build().unwrap();
        let mut app = App::<R>::new();
        event_loop.run_app_on_demand(&mut app).unwrap()
    }
}
