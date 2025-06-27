#![feature(portable_simd)]
#![windows_subsystem = "windows"]
mod component;
mod main_loop;
mod object;
mod render;
mod scene;
mod transform;
mod utils;
use std::{collections::HashMap, io::Cursor};

use env_logger::Target::Stdout;
use rodio::{Decoder, OutputStream, OutputStreamHandle, Sink, Source};
use winit::{
    application::ApplicationHandler,
    event::{ElementState::Pressed, KeyEvent, WindowEvent},
    event_loop::ActiveEventLoop,
    keyboard::{Key, NamedKey},
    window::{Window, WindowAttributes, WindowId},
};

use crate::main_loop::{MainLoop, WinitMainLoop};
struct App {
    windows:   HashMap<WindowId, Window>,
    app_sound: AppSound,
}
struct AppSound {
    // source: Buffered<LoopedDecoder<Cursor<&'static [u8]>>>,
    stream:        OutputStream,
    stream_handle: OutputStreamHandle,
    sink:          Sink,
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
impl App {
    fn new() -> Self { Self { windows: HashMap::new(), app_sound: AppSound::new() } }
}
impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        self.windows.clear();
        let wa = WindowAttributes::default()
            .with_blur(true)
            .with_min_inner_size(winit::dpi::LogicalSize::new(400.0, 300.0))
            .with_resizable(true)
            .with_transparent(true)
            .with_title(env!("CARGO_PKG_DESCRIPTION"));
        let window = event_loop.create_window(wa).unwrap();
        self.windows.insert(window.id(), window);
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
#[tokio::main]
async fn main() {
    env_logger::builder().target(Stdout).init();
    let mainloop = WinitMainLoop;
    mainloop.run().await
}
