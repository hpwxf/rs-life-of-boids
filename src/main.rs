use glutin::event::{Event, WindowEvent, VirtualKeyCode, ElementState, KeyboardInput, StartCause};
use glutin::event_loop::{ControlFlow, EventLoop};
use glutin::window::{WindowBuilder, Fullscreen};
use glutin::ContextBuilder;
use glutin::monitor::{MonitorHandle};

mod support;

fn main() {
    let events_loop = EventLoop::new();

    let fullscreen = Some(Fullscreen::Borderless(Some(prompt_for_monitor(&events_loop))));
    let mut is_maximized = false;
    let mut decorations = true;

    let wb = WindowBuilder::new()
        .with_title("A fantastic window!")
        // .with_transparent(true)
        // .with_decorations(true)
        ;

    let windowed_context = ContextBuilder::new().build_windowed(wb, &events_loop).unwrap();
    let windowed_context = unsafe { windowed_context.make_current().unwrap() };
    
    
    println!("Info:");
    println!("\tAPI: {:?}", windowed_context.get_api());
    println!("\tPixel format of the window's GL context: {:?}", windowed_context.get_pixel_format());
    println!("\twindow size: {:?}",windowed_context.window().outer_size());

    let gl = support::load(&windowed_context.context());

    let start_time = std::time::SystemTime::now();

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
                    gl.draw_frame(elapsed.as_secs_f32(), ratio, [0.0, 0.0, 0.0, 0.0]);
                    windowed_context.swap_buffers().unwrap();
                }
            }
            _ => (),
        }
    });
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
