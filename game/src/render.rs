use ::wgpu::SurfaceTarget;
use winit::dpi::LogicalSize;
use winit::window::Window;
pub mod wgpu;
pub trait Renderer {
    type Surface<'a> where Self: 'a;
    fn new() -> Self;
    async fn create_surface<'window>(&self, size: LogicalSize<u32>, window: impl Into<SurfaceTarget<'window>>) -> Self::Surface<'window> where Self: 'window;
}
