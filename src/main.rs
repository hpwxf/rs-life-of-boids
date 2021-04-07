use glutin::event::{Event, WindowEvent, VirtualKeyCode, ElementState, KeyboardInput, StartCause};
use glutin::event_loop::{ControlFlow, EventLoop};
use glutin::window::{WindowBuilder, Fullscreen, Window};
use glutin::{ContextBuilder, PossiblyCurrent, ContextWrapper};
use glutin::monitor::{MonitorHandle};

#[macro_use]
mod support;
mod glx;
mod render;
mod fps;

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
    let (windowed_context, fullscreen) = make_window_context(&events_loop, window_config);
    print_debug_info(&windowed_context);

    let mut is_maximized = false;
    let mut decorations = true;

    let start_time = std::time::SystemTime::now();

    let mut fps_counter = FpsCounter::new();
    let mut fps_cacher = FpsCache::new(CACHE_FPS_MS);

    let window_info = glx::get_window_size_info(windowed_context.window()).unwrap_or_else(|_| panic!(""));
    let renderer_config = RendererConfig { width: window_info.width, height: window_info.height, boid_size: 1.0 };

    let gl = gl_init(&windowed_context);
    let mut renderer = Renderer::new(gl, renderer_config);
    renderer.initialize();

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
                        println!("window.fullscreen {:?}", windowed_context.window().fullscreen());
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
                    renderer.draw_frame(elapsed.as_secs_f32(), ratio, [0.0, 0.0, 0.0, 0.0]);


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

fn make_window_context(events_loop: &EventLoop<()>, window_config: WindowConfig) -> (ContextWrapper<PossiblyCurrent, Window>, Option<Fullscreen>) {
    let fullscreen = Some(Fullscreen::Borderless(Some(prompt_for_monitor(&events_loop))));

    let wb = WindowBuilder::new()
        .with_title(TITLE)
        // .with_transparent(true)
        // .with_decorations(true)
        ;

    let windowed_context = ContextBuilder::new().build_windowed(wb, &events_loop).unwrap();
    let windowed_context: ContextWrapper<PossiblyCurrent, Window> = unsafe { windowed_context.make_current().unwrap() };
    (windowed_context, fullscreen)
}


// Enumerate monitors and prompt user to choose one
fn prompt_for_monitor(el: &EventLoop<()>) -> MonitorHandle {
    // for (num, monitor) in el.available_monitors().enumerate() {
    //     println!("Monitor #{}: {:?}", num, monitor.name());
    // }

    // print!("Please write the number of the monitor to use: ");
    // use std::io::Write;
    // std::io::stdout().flush().unwrap();
    // 
    // let mut num = String::new();
    // std::io::stdin().read_line(&mut num).unwrap();
    // let num = num.trim().parse().ok().expect("Please enter a number");
    let num = 0;
    let monitor = el.available_monitors().nth(num).expect("Please enter a valid ID");

    println!("Using {:?}", monitor.name());

    monitor
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

use support::gl;
use std::ffi::CStr;
use crate::fps::{FpsCounter, FpsCache};
use crate::render::{RendererConfig, Renderer};

fn gl_init(windowed_context: &ContextWrapper<PossiblyCurrent, Window>) -> gl::Gl {
    let gl_context = windowed_context.context();
    let gl = gl::Gl::load_with(|ptr| gl_context.get_proc_address(ptr) as *const _);

    let version = unsafe {
        let data = CStr::from_ptr(gl.GetString(gl::VERSION) as *const _).to_bytes().to_vec();
        String::from_utf8(data).unwrap()
    };

    println!("OpenGL version {}", version);
    gl
}