use wgpu::{Device, Instance, Queue, Surface, SurfaceTarget, TextureFormat};
use wgpu::rwh::HasWindowHandle;
use winit::dpi::LogicalSize;
use winit::window::Window;

use crate::render::Renderer;
struct WGPURender {
    instance: Instance,
}
struct WGPUSurface<'a> {
    surface: Surface<'a>,
    device:  Device,
    queue:   Queue,
}
impl Renderer for WGPURender {
    type Surface<'a> = WGPUSurface<'a>;

    fn new() -> Self {
        let instance = Instance::default();
        Self { instance }
    }

    async fn create_surface<'window>(&self, size: LogicalSize<u32>, window: impl Into<SurfaceTarget<'window>>) -> Self::Surface<'window> {
        let surface = self.instance.create_surface(window).expect("Failed to create surface");
        let adapter = self
            .instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference:       wgpu::PowerPreference::HighPerformance,
                compatible_surface:     Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .unwrap();
        let (device, queue) =
            adapter.request_device(&wgpu::DeviceDescriptor::default()).await.unwrap();
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface.get_capabilities(&adapter).formats[0],
            width: size.width.max(1),
            height: size.height.max(1),
            present_mode: wgpu::PresentMode::Fifo,
            desired_maximum_frame_latency: 0,
            alpha_mode: Default::default(),
            view_formats: vec![TextureFormat::Rgba8UnormSrgb],
        };
        surface.configure(&device, &config);
        Self::Surface { surface, device, queue }
    }
}
