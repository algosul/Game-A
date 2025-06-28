#![feature(portable_simd)]
#![windows_subsystem = "windows"]
mod component;
mod main_loop;
mod object;
mod render;
mod scene;
mod transform;
mod utils;
use std::{collections::HashMap, io::Cursor, sync::Arc};

use env_logger::Target::Stdout;
use rodio::{Decoder, OutputStream, OutputStreamHandle, Sink, Source};
use tokio::runtime::Runtime;
use winit::{
    application::ApplicationHandler,
    event::{ElementState::Pressed, KeyEvent, WindowEvent},
    event_loop::ActiveEventLoop,
    keyboard::{Key, NamedKey},
    window::{Window, WindowAttributes, WindowId},
};

use crate::{
    main_loop::{MainLoop, WinitMainLoop},
    render::{wgpu::WGPUSurface, Surface},
};
struct App<R: Surface> {
    windows:   HashMap<WindowId, AppWindow<R>>,
    app_sound: AppSound,
}
struct AppSound {
    // source: Buffered<LoopedDecoder<Cursor<&'static [u8]>>>,
    stream:        OutputStream,
    stream_handle: OutputStreamHandle,
    sink:          Sink,
}
struct AppWindow<R: Surface> {
    window:  Arc<Window>,
    surface: R,
}
impl AppSound {
    fn new() -> AppSound {
        let (stream, stream_handle) = OutputStream::try_default().unwrap();
        let sink = Sink::try_new(&stream_handle).unwrap();
        const FILE: &[u8] = include_bytes!("../rc/music/CASE WANG - Cyberpunk.mp3");
        let source = Decoder::new_looped(Cursor::new(FILE)).unwrap().buffered();
        sink.append(source);
        sink.pause();
        sink.set_volume(0.5);
        Self { stream, stream_handle, sink }
    }

    fn play(&self) { self.sink.play(); }
}
impl<R: Surface> App<R> {
    fn new() -> Self { Self { windows: HashMap::new(), app_sound: AppSound::new() } }

    fn new_window(
        &self, event_loop: &ActiveEventLoop, wa: WindowAttributes,
    ) -> (WindowId, AppWindow<R>) {
        let window = Arc::new(event_loop.create_window(wa).unwrap());
        let id = window.id();
        let inner_size = window.inner_size();
        let runtime = Runtime::new().unwrap();
        let surface = runtime.block_on(async { R::new(inner_size, window.clone()).await });
        (id, AppWindow { window, surface })
    }
}
impl<R: Surface> ApplicationHandler for App<R> {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        self.windows.clear();
        let wa = WindowAttributes::default()
            .with_blur(true)
            .with_min_inner_size(winit::dpi::LogicalSize::new(400.0, 300.0))
            .with_resizable(true)
            .with_transparent(true)
            .with_title(env!("CARGO_PKG_DESCRIPTION"));
        let (id, window) = self.new_window(event_loop, wa);
        self.windows.insert(id, window);
        self.app_sound.play();
    }

    fn window_event(
        &mut self, event_loop: &ActiveEventLoop, window_id: WindowId, event: WindowEvent,
    ) {
        let _window = match self.windows.get_mut(&window_id) {
            Some(window) => window,
            None => return,
        };
        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            WindowEvent::KeyboardInput {
                device_id: _device_id,
                event: KeyEvent { logical_key, state, .. },
                is_synthetic: _is_synthetic,
            } => match (logical_key, state) {
                (Key::Named(NamedKey::Space), Pressed) => {}
                (Key::Named(NamedKey::Escape), Pressed) => {
                    event_loop.exit();
                }
                _ => {}
            },
            _ => {}
        }
    }
}
fn main() {
    env_logger::builder().target(Stdout).init();
    let mainloop = WinitMainLoop;
    mainloop.run::<WGPUSurface>()
}
