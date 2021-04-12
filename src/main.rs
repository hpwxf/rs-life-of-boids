use glutin::event::{ElementState, Event, KeyboardInput, StartCause, VirtualKeyCode, WindowEvent};
use glutin::event_loop::{ControlFlow, EventLoop};
use glutin::window::{Fullscreen, Window, WindowBuilder};
use glutin::{ContextBuilder, ContextWrapper, PossiblyCurrent};

#[allow(unused_imports)]
use crate::fps::{FpsCache, FpsCounter};
use crate::render::{Renderer, RendererConfig};
use glutin::dpi::PhysicalSize;
use std::path::PathBuf;

use anyhow::{anyhow,Result};
use crate::points_simulator::PointsSimulator;

#[macro_use]
mod glx;
mod utils;
mod fps;
mod render;
mod shader_programs;
mod points_simulator;

const TITLE: &str = "new rusty boids";
// const CACHE_FPS_MS: u64 = 500;

pub enum WindowConfig {
    Fullscreen,
    Dimensions((u32, u32)),
    Default,
}

fn main() -> Result<()> {
    let events_loop = EventLoop::new();

    let monitor = events_loop
        .available_monitors()
        .nth(1).ok_or(anyhow!("Cannot use this monitor"))?;
    let fullscreen = Some(Fullscreen::Borderless(Some(monitor)));

    let wb = WindowBuilder::new()
        .with_title(TITLE)
        // .with_transparent(true)
        // .with_decorations(true)
        ;
    let windowed_context = ContextBuilder::new()
        .with_vsync(true)
        .build_windowed(wb, &events_loop)
        .unwrap();
    let windowed_context: ContextWrapper<PossiblyCurrent, Window> =
        unsafe { windowed_context.make_current().unwrap() };
    print_debug_info(&windowed_context);

    let mut is_maximized = false;
    let mut decorations = true;

    let start_time = std::time::SystemTime::now();

    // let mut fps_counter = FpsCounter::new();
    // let mut fps_cacher = FpsCache::new(CACHE_FPS_MS);

    let mut last_time = std::time::Instant::now();
    let mut count = 0;
    let mut accumulated_time = 0.0;

    let window_info =
        glx::get_window_size_info(windowed_context.window()).expect("Cannot get window size info");
    let renderer_config = RendererConfig { size: window_info };

    let gl = glx::gl_init(&windowed_context);

    let mut renderer = Renderer::new(gl, renderer_config)?;
    renderer.initialize()?;

    println!("Current dir = {:?}", std::env::current_dir());

    let mut s = PointsSimulator::new(window_info);
    
    events_loop.run(move |event, _, control_flow| {
        // println!("{:?}", event);
        // *control_flow = ControlFlow::Wait; // no auto refresh
        *control_flow = ControlFlow::Poll;

        match event {
            Event::LoopDestroyed => return,
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::Resized(physical_size) => windowed_context.resize(physical_size),
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                WindowEvent::KeyboardInput {
                    input:
                        KeyboardInput {
                            virtual_keycode: Some(virtual_code),
                            state,
                            ..
                        },
                    ..
                } => match (virtual_code, state) {
                    (VirtualKeyCode::Escape, _) | (VirtualKeyCode::Q, _) => {
                        *control_flow = ControlFlow::Exit
                    }
                    (VirtualKeyCode::F, ElementState::Pressed) => {
                        if windowed_context.window().fullscreen().is_some() {
                            windowed_context.window().set_fullscreen(None);
                        } else {
                            windowed_context.window().set_fullscreen(fullscreen.clone());
                        }
                    }
                    (VirtualKeyCode::S, ElementState::Pressed) => {
                        println!(
                            "Fullscreen info: {:?}",
                            windowed_context.window().fullscreen()
                        );
                        // println!("FPS info: {:?}", fps_counter.average_fps());
                        println!(
                            "ScaleFactor info: {:?}",
                            windowed_context.window().scale_factor()
                        );
                        glx::save_image(
                            renderer.gl.clone(),
                            &PathBuf::from("export.png"),
                            &windowed_context.window(),
                        );
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
                },
                _ => (),
            },
            Event::RedrawRequested(_) | Event::NewEvents(StartCause::Poll) => {
                if let Ok(elapsed) = start_time.elapsed() {
                    let PhysicalSize { width, height } = windowed_context.window().inner_size();
                    let ratio = width as f32 / height as f32;

                    s.update((width, height));
                    let points = &s.points;
                    
                    renderer
                        .render(
                            elapsed.as_secs_f32(),
                            ratio,
                            [0.0, 0.0, 0.0, 0.0],
                            &points,
                            (width, height),
                        )
                        .unwrap();
                    
                    
                    windowed_context.swap_buffers().unwrap();
                }
            }
            _ => (),
        }

        let now = std::time::Instant::now();
        let duration = now.duration_since(last_time);
        last_time = now;
        let elapsed_time =
            duration.as_secs() as f64 * 1000.0 + duration.subsec_nanos() as f64 * 1e-6;
        accumulated_time += elapsed_time;
        count += 1;
        if accumulated_time > 1000.0 {
            let title = format!("FPS: {:.2}", count as f64 / (accumulated_time * 0.001));
            windowed_context.window().set_title(title.as_str());
            count = 0;
            accumulated_time = 0.0;
        }

        // fps_counter.tick();
        // fps_cacher.poll(&fps_counter, |new_fps| {
        //     let title = format!("{} - {:02} fps", TITLE, new_fps);
        //     windowed_context.window().set_title(&title);
        // });
    });
}

fn print_debug_info(windowed_context: &ContextWrapper<PossiblyCurrent, Window>) {
    println!("Info:");
    println!("\tAPI: {:?}", windowed_context.get_api());
    println!(
        "\tPixel format of the window's GL context: {:?}",
        windowed_context.get_pixel_format()
    );
    println!(
        "\twindow size: {:?}",
        windowed_context.window().outer_size()
    );

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
