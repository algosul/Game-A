use std::{collections::HashMap, io::Cursor};

use env_logger::Target::Stdout;
use rodio::{
    source::Buffered,
    Decoder,
    OutputStream
    ,
    OutputStreamHandle

    ,
    Source,
};
use winit::{
    application::ApplicationHandler,
    event::{ElementState::Pressed, KeyEvent, WindowEvent},
    event_loop::{ActiveEventLoop, EventLoop},
    keyboard::{Key, NamedKey},
    window::{Window, WindowAttributes, WindowId},
};
struct App {
    windows:   HashMap<WindowId, Window>,
    app_sound: AppSound,
}
struct AppSound {
    source: Buffered<Decoder<Cursor<&'static [u8]>>>,
    stream: OutputStream,
    handle: OutputStreamHandle,
}
impl AppSound {
    fn new() -> AppSound {
        let (stream, handle) = OutputStream::try_default().unwrap();
        const FILE: &[u8] = include_bytes!("../rc/music/CASE WANG - Cyberpunk.mp3");
        let source = Decoder::new_mp3(Cursor::new(FILE)).unwrap().buffered();
        Self { source, stream, handle }
    }

    fn play(&self) { self.handle.play_raw(self.source.clone().convert_samples()).unwrap(); }
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
        let window = match self.windows.get_mut(&window_id) {
            Some(window) => window,
            None => return,
        };
        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            WindowEvent::KeyboardInput {
                device_id,
                event: KeyEvent { logical_key, state, .. },
                is_synthetic,
            } => match (logical_key, state) {
                (Key::Named(NamedKey::Space), Pressed) => {
                    
                }
                (Key::Named(NamedKey::Escape), Pressed) => {
                    event_loop.exit();
                }
                _ => {}
            },
            _ => {}
        }
    }
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::builder().target(Stdout).init();
    let event_loop = EventLoop::new()?;
    let mut app = App::new();
    event_loop.run_app(&mut app)?;
    Ok(())
}
