use error_iter::ErrorIter as _;
use log::error;
use pixels::{Error, Pixels, SurfaceTexture};
use std::time::Instant;
use winit::dpi::LogicalSize;
use winit::error::OsError;
use winit::event::{Event, WindowEvent};
use winit::event_loop::EventLoop;
use winit::keyboard::KeyCode;
use winit::window::{Window, WindowBuilder};
use winit_input_helper::WinitInputHelper;

use crate::world::World;

mod branch;
mod limits;
mod tree;
mod types;
mod world;

const WIDTH: u32 = 1024;
const HEIGHT: u32 = 768;
const TIME_SCALE: f64 = 4.0;

fn main() -> Result<(), Error> {
    env_logger::init();
    let event_loop = EventLoop::new().unwrap();
    let window = build_window(WIDTH, HEIGHT, &event_loop).expect("Failed to create window");
    let mut pixels = build_pixels(WIDTH, HEIGHT, &window).expect("Failed to pixels");
    let mut input = WinitInputHelper::new();
    let mut world = World::new(WIDTH, HEIGHT);
    let mut last_update_time = Instant::now();

    let res = event_loop.run(|event, elwt| {
        if redraw(&event) {
            // Redraw frame
            world.draw(pixels.frame_mut());
            if let Err(err) = pixels.render() {
                log_error("pixels.render", err);
                elwt.exit();
                return;
            }
        }

        if input.update(&event) {
            // Close
            if input.key_pressed(KeyCode::Escape) || input.close_requested() {
                elwt.exit();
                return;
            }

            // Resize
            if let Some(size) = input.window_resized() {
                if let Err(err) = pixels.resize_surface(size.width, size.height) {
                    log_error("pixels.resize_surface", err);
                    elwt.exit();
                    return;
                }
            }

            let time_delta = last_update_time.elapsed().as_secs_f64() * TIME_SCALE;
            world.update(time_delta);
            last_update_time = Instant::now();

            window.request_redraw();
        }
    });
    res.map_err(|e| Error::UserDefined(Box::new(e)))
}

fn redraw(event: &Event<()>) -> bool {
    match event {
        Event::WindowEvent {
            event: WindowEvent::RedrawRequested,
            ..
        } => true,
        _ => false,
    }
}

fn build_window(width: u32, height: u32, event_loop: &EventLoop<()>) -> Result<Window, OsError> {
    let size = LogicalSize::new(width as f64, height as f64);
    WindowBuilder::new()
        .with_title("Tree")
        .with_inner_size(size)
        .with_min_inner_size(size)
        .build(&event_loop)
}

fn build_pixels(width: u32, height: u32, window: &Window) -> Result<Pixels, Error> {
    let surface_texture = SurfaceTexture::new(width, height, window);
    Pixels::new(width, height, surface_texture)
}

fn log_error<E: std::error::Error + 'static>(method_name: &str, err: E) {
    error!("{method_name}() failed: {err}");
    for source in err.sources().skip(1) {
        error!("  Caused by: {source}");
    }
}
