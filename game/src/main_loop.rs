use std::{
    collections::HashMap,
    io::Cursor,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
        Mutex,
        RwLock,
    },
};

use log::warn;
use rodio::{Decoder, OutputStream, OutputStreamHandle, Sink, Source};
use tokio::runtime::Runtime;
use winit::{
    application::ApplicationHandler,
    dpi::PhysicalSize,
    event::{ElementState::Pressed, KeyEvent, WindowEvent},
    event_loop::ActiveEventLoop,
    keyboard::{Key, NamedKey},
    window::{Window, WindowAttributes, WindowId},
};

use crate::{render::Surface, scene::scenes::DynamicScene, world::World};
pub trait MainLoop {
    fn new() -> Self;
    fn run<S: Surface>(self);
}
struct WInitApp<S: Surface> {
    ctxs:                  HashMap<WindowId, WInitCtx<S>>,
    background_music_sink: BackgroundMusicSink,
}
struct BackgroundMusicSink {
    stream:        OutputStream,
    stream_handle: OutputStreamHandle,
    sink:          Sink,
}
struct WInitCtx<S: Surface> {
    window:  Arc<Window>,
    surface: S,
}
impl<S: Surface> WInitCtx<S> {
    fn window(&self) -> &Window { self.window.as_ref() }

    fn redraw(&mut self) { self.surface.draw(); }

    fn resize(&mut self, physical_size: PhysicalSize<u32>) {
        self.surface.resize(physical_size);
    }
}
pub struct WInitMainLoop {
    world: Arc<RwLock<World>>,
}
impl BackgroundMusicSink {
    fn new() -> BackgroundMusicSink {
        let (stream, stream_handle) = OutputStream::try_default().unwrap();
        let sink = Sink::try_new(&stream_handle).unwrap();
        const FILE: &[u8] =
            include_bytes!("../rc/music/CASE WANG - Cyberpunk.mp3");
        let source = Decoder::new_looped(Cursor::new(FILE)).unwrap().buffered();
        sink.append(source);
        sink.pause();
        Self { stream, stream_handle, sink }
    }

    fn set_volume(&self, volume: f32) { self.sink.set_volume(volume); }

    fn play(&self) { self.sink.play(); }

    fn pause(&self) { self.sink.pause(); }
}
impl<S: Surface> WInitApp<S> {
    fn new() -> Self {
        Self {
            ctxs:                  HashMap::new(),
            background_music_sink: BackgroundMusicSink::new(),
        }
    }

    async fn new_ctxs(window: Window) -> WInitCtx<S> {
        let window = Arc::new(window);
        let surface = S::new(window.inner_size(), window.clone()).await;
        WInitCtx { window, surface }
    }

    async fn new_and_add_ctxs(&mut self, window: Window) {
        self.ctxs.insert(window.id(), Self::new_ctxs(window).await);
    }
}
impl<S: Surface> ApplicationHandler for WInitApp<S> {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        self.background_music_sink.set_volume(0.3);
        let wa = WindowAttributes::default()
            .with_blur(true)
            .with_min_inner_size(winit::dpi::LogicalSize::new(400.0, 300.0))
            .with_resizable(true)
            .with_transparent(true)
            .with_title(env!("CARGO_PKG_DESCRIPTION"));
        Runtime::new().unwrap().block_on(async {
            self.new_and_add_ctxs(event_loop.create_window(wa).unwrap()).await
        });
    }

    fn window_event(
        &mut self, event_loop: &ActiveEventLoop, window_id: WindowId,
        event: WindowEvent,
    ) {
        let ctx = self.ctxs.get_mut(&window_id).unwrap();
        match event {
            WindowEvent::Focused(false) => {
                self.background_music_sink.pause();
            }
            WindowEvent::Focused(true) => {
                self.background_music_sink.play();
            }
            WindowEvent::Resized(physical_size) => {
                if physical_size.width == 0 || physical_size.height == 0 {
                } else {
                    ctx.resize(physical_size);
                }
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
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                ctx.redraw();
            }
            WindowEvent::Touch(winit::event::Touch {
                device_id: _device_id,
                id: _id,
                force: _force,
                location: _location,
                phase: _phase,
            }) => {}
            _ => {}
        }
    }

    fn suspended(&mut self, _event_loop: &ActiveEventLoop) {
        warn!("APP suspended");
    }
}
impl MainLoop for WInitMainLoop {
    fn new() -> Self { Self { world: Arc::new(RwLock::new(World::new())) } }

    fn run<S: Surface>(self) {
        use std::thread::spawn;
        let is_running = Arc::new(AtomicBool::new(true));
        let main_loop = || {
            let event_loop =
                winit::event_loop::EventLoop::builder().build().unwrap();
            let mut app = WInitApp::<S>::new();
            event_loop.run_app(&mut app).unwrap();
        };
        let game_loop = {
            let is_running = is_running.clone();
            let world = self.world.clone();
            spawn(move || {
                world
                    .write()
                    .unwrap()
                    .load_scene(Arc::new(Mutex::new(DynamicScene::new())));
                let world = world;
                while is_running.load(Ordering::SeqCst) {
                    world.read().unwrap().update();
                }
            })
        };
        let render_loop = {
            let is_running = is_running.clone();
            let world = self.world.clone();
            spawn(move || {
                while is_running.load(Ordering::SeqCst) {
                    world.read().unwrap().draw();
                }
            })
        };
        main_loop();
        is_running.store(false, Ordering::SeqCst);
        game_loop.join().unwrap();
        render_loop.join().unwrap();
    }
}
