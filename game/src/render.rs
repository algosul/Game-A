use std::sync::Arc;

use winit::{dpi::PhysicalSize, window::Window};
pub mod wgpu;
pub trait Surface {
    async fn new(size: PhysicalSize<u32>, window: Arc<Window>) -> Self
    where Self: Sized;
}
pub enum RendererType {
    WGPU,
}
