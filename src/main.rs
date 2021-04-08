use glutin::{ContextBuilder, ContextWrapper, PossiblyCurrent};
use glutin::event::{ElementState, Event, KeyboardInput, StartCause, VirtualKeyCode, WindowEvent};
use glutin::event_loop::{ControlFlow, EventLoop};
use glutin::window::{Fullscreen, Window, WindowBuilder};

use crate::fps::{FpsCache, FpsCounter};
use crate::render::{Renderer, RendererConfig};
use crate::shaders::points::Point;
use cgmath::{Vector2, Point2, Rad, Basis2, Rotation2, Rotation, Zero};
use rand::distributions::{Range, IndependentSample};

#[macro_use]
mod support;
mod glx;
mod render;
mod fps;
mod shaders;

const TITLE: &str = "new rusty boids";
const CACHE_FPS_MS: u64 = 500;

pub enum WindowConfig {
    Fullscreen,
    Dimensions((u32, u32)),
    Default,
}

fn main() {
    let events_loop = EventLoop::new();
    let window_config = WindowConfig::Default;

    let monitor = events_loop.available_monitors().nth(1).expect("This monitor is not available");
    let fullscreen = Some(Fullscreen::Borderless(Some(monitor)));

    let wb = WindowBuilder::new()
        .with_title(TITLE)
        // .with_transparent(true)
        // .with_decorations(true)
        ;
    let windowed_context = ContextBuilder::new().build_windowed(wb, &&events_loop).unwrap();
    let windowed_context: ContextWrapper<PossiblyCurrent, Window> = unsafe { windowed_context.make_current().unwrap() };
    print_debug_info(&windowed_context);

    let mut is_maximized = false;
    let mut decorations = true;

    let start_time = std::time::SystemTime::now();

    let mut fps_counter = FpsCounter::new();
    let mut fps_cacher = FpsCache::new(CACHE_FPS_MS);

    let window_info = glx::get_window_size_info(windowed_context.window()).expect("Cannot get window size info");
    let renderer_config = RendererConfig { width: window_info.width, height: window_info.height };

    let gl = glx::gl_init(&windowed_context);

    let mut renderer = Renderer::new(gl, renderer_config);
    renderer.initialize();

    let mut points = Vec::<Point>::with_capacity(10000);
    let get_pos = |t: f32| {
        Point2 {
            x: window_info.width * (0.5 + 0.4 * f32::cos(t)),
            y: window_info.height * (0.5 + 0.4 * f32::sin(t)),
        }
    };

    let mut v: f32 = 0.0;
    points.resize_with(points.capacity(), || {
        v += 1.0;
        Point {
            position: get_pos(v),
            velocity: Vector2::zero(),
        }
    });
    let mut rng = rand::thread_rng();

    events_loop.run(move |event, _, control_flow| {
        // println!("{:?}", event);
        // *control_flow = ControlFlow::Wait; // no auto refresh
        *control_flow = ControlFlow::Poll;

        match event {
            Event::LoopDestroyed => return,
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::Resized(physical_size) => { windowed_context.resize(physical_size) }
                WindowEvent::CloseRequested => { *control_flow = ControlFlow::Exit }
                WindowEvent::KeyboardInput {
                    input: KeyboardInput { virtual_keycode: Some(virtual_code), state, .. },
                    ..
                } => match (virtual_code, state) {
                    (VirtualKeyCode::Escape, _) | (VirtualKeyCode::Q, _) => *control_flow = ControlFlow::Exit,
                    (VirtualKeyCode::F, ElementState::Pressed) => {
                        if windowed_context.window().fullscreen().is_some() {
                            windowed_context.window().set_fullscreen(None);
                        } else {
                            windowed_context.window().set_fullscreen(fullscreen.clone());
                        }
                    }
                    (VirtualKeyCode::S, ElementState::Pressed) => {
                        println!("Fullscreen info: {:?}", windowed_context.window().fullscreen());
                        println!("FPS info: {:?}", fps_counter.average_fps());
                    }
                    (VirtualKeyCode::M, ElementState::Pressed) => {
                        is_maximized = !is_maximized;
                        windowed_context.window().set_maximized(is_maximized);
                    }
                    (VirtualKeyCode::D, ElementState::Pressed) => {
                        decorations = !decorations;
                        windowed_context.window().set_decorations(decorations);
                    }
                    _ => (),
                }
                _ => (),
            },
            Event::RedrawRequested(_) | Event::NewEvents(StartCause::Poll) => {
                if let Ok(elapsed) = start_time.elapsed() {
                    let physical_size = windowed_context.window().inner_size();
                    let ratio = physical_size.width as f32 / physical_size.height as f32;
                    ///////////////////
                    let vel_space = Range::new(0., 10.0);
                    let ang_space = Range::new(0., 6.28);
                    for p in &mut points {
                        let a = ang_space.ind_sample(&mut rng);
                        let m = vel_space.ind_sample(&mut rng);
                        p.velocity = Basis2::from_angle(Rad(a)).rotate_vector(Vector2::new(0., m));
                        p.position += p.velocity / 5.0;
                    }
                    ///////////////////
                    renderer.render(elapsed.as_secs_f32(), ratio, [0.0, 0.0, 0.0, 0.0], &window_info, &points);
                    windowed_context.swap_buffers().unwrap();
                }
            }
            _ => (),
        }

        fps_counter.tick();
        fps_cacher.poll(&fps_counter, |new_fps| {
            let title = format!("{} - {:02} fps", TITLE, new_fps);
            windowed_context.window().set_title(&title);
        });
    });
}

fn print_debug_info(windowed_context: &ContextWrapper<PossiblyCurrent, Window>) {
    println!("Info:");
    println!("\tAPI: {:?}", windowed_context.get_api());
    println!("\tPixel format of the window's GL context: {:?}", windowed_context.get_pixel_format());
    println!("\twindow size: {:?}", windowed_context.window().outer_size());

    // println!("Vendor: {}", glx::get_gl_str(gl::VENDOR));
    // println!("Renderer: {}", glx::get_gl_str(gl::RENDERER));
    // println!("Version: {}", glx::get_gl_str(gl::VERSION));
    // println!(
    //     "GLSL version: {}",
    //     glx::get_gl_str(gl::SHADING_LANGUAGE_VERSION)
    // );
    // println!("Extensions: {}", glx::get_gl_extensions().join(","));
    // println!("Hidpi factor: {}", window.get_hidpi_factor());
}

