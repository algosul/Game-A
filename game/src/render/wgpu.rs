use std::sync::Arc;

use wgpu::{Device, Queue, Surface, TextureFormat};
use winit::{dpi::PhysicalSize, window::Window};
pub struct WGPUSurface<'w> {
    window:  Arc<Window>,
    surface: Surface<'w>,
    device:  Device,
    queue:   Queue,
}
impl<'w> super::Surface for WGPUSurface<'w> {
    async fn new(size: PhysicalSize<u32>, window: Arc<Window>) -> Self {
        let instance = wgpu::Instance::default();
        let surface = instance.create_surface(window.clone()).expect("Failed to create surface");
        let adapter = instance
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
        Self { window, surface, device, queue }
    }
}
