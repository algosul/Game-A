use std::sync::Arc;

use log::info;
use wgpu::{
    wgt::{CommandEncoderDescriptor, TextureViewDescriptor},
    Adapter,
    Color,
    Device,
    DeviceDescriptor,
    Features,
    Limits,
    LoadOp,
    Operations,
    PresentMode,
    Queue,
    RenderPassColorAttachment,
    RenderPassDescriptor,
    StoreOp,
    Surface,
    SurfaceConfiguration,
    TextureFormat,
};
use winit::{dpi::PhysicalSize, window::Window};
pub struct WGPUSurface<'w> {
    window:  Arc<Window>,
    surface: Surface<'w>,
    device:  Device,
    adapter: Adapter,
    queue:   Queue,
}
impl<'w> WGPUSurface<'w> {
    fn config(
        size: PhysicalSize<u32>, surface: &Surface, adapter: &Adapter,
    ) -> SurfaceConfiguration {
        let caps = surface.get_capabilities(adapter);
        let modes = surface.get_capabilities(adapter).present_modes;
        info!("Support present mode: {modes:?}");
        let formats = surface.get_capabilities(adapter).formats;
        info!("Support format: {formats:?}");
        SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: caps.formats[0],
            width: size.width,
            height: size.height,
            present_mode: PresentMode::AutoNoVsync,
            alpha_mode: caps.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        }
    }

    fn configure(&self, size: PhysicalSize<u32>) {
        let config = Self::config(size, &self.surface, &self.adapter);
        self.surface.configure(&self.device, &config);
    }
}
impl<'w> super::Surface for WGPUSurface<'w> {
    async fn new(size: PhysicalSize<u32>, window: Arc<Window>) -> Self {
        let instance = wgpu::Instance::default();
        let surface = instance.create_surface(window.clone()).expect("Failed to create surface");
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference:       wgpu::PowerPreference::default(),
                compatible_surface:     Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .unwrap();
        let (device, queue) = adapter
            .request_device(&DeviceDescriptor {
                label:             None,
                required_features: Features::empty(),
                required_limits:   Limits::downlevel_webgl2_defaults()
                    .using_resolution(adapter.limits()),
                memory_hints:      Default::default(),
                trace:             Default::default(),
            })
            .await
            .unwrap();
        let config = Self::config(size, &surface, &adapter);
        surface.configure(&device, &config);
        Self { window, surface, device, adapter, queue }
    }

    fn resize(&mut self, size: PhysicalSize<u32>) { self.configure(size); }

    fn draw(&mut self) {
        let surface_texture = self
            .surface
            .get_current_texture()
            .expect("Failed to acquire next swap chain texture");
        let texture_view = surface_texture.texture.create_view(&TextureViewDescriptor::default());
        let mut encoder =
            self.device.create_command_encoder(&CommandEncoderDescriptor { label: None });
        encoder.begin_render_pass(&RenderPassDescriptor {
            label:                    None,
            color_attachments:        &[Some(RenderPassColorAttachment {
                view:           &texture_view,
                resolve_target: None,
                ops:            Operations {
                    load:  LoadOp::Clear(Color::TRANSPARENT),
                    store: StoreOp::Store,
                },
            })],
            depth_stencil_attachment: None,
            timestamp_writes:         None,
            occlusion_query_set:      None,
        });
        self.queue.submit(Some(encoder.finish()));
        surface_texture.present();
    }
}
