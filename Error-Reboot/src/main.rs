#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use env_logger::Target::Stdout;
use game::{
    main_loop::{MainLoop, WInitMainLoop},
    render::wgpu::WGPUSurface,
};
use log::LevelFilter;
fn main() {
    env_logger::builder()
        .target(Stdout)
        .filter_module("wgpu_core", LevelFilter::Info)
        .filter_module("naga", LevelFilter::Info)
        .init();
    let mainloop = WInitMainLoop::new();
    mainloop.run::<WGPUSurface>()
}
