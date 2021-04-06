use glutin::event::{Event, WindowEvent, VirtualKeyCode, ElementState, KeyboardInput};
use glutin::event_loop::{ControlFlow, EventLoop};
use glutin::window::{WindowBuilder, Fullscreen};
use glutin::ContextBuilder;
use glutin::monitor::{MonitorHandle};

mod support;

fn main() {
    let el = EventLoop::new();

    let fullscreen = Some(Fullscreen::Borderless(Some(prompt_for_monitor(&el))));
    let mut is_maximized = false;
    let mut decorations = true;

    let wb = WindowBuilder::new()
        .with_title("A fantastic window!");

    let windowed_context = ContextBuilder::new().build_windowed(wb, &el).unwrap();

    let windowed_context = unsafe { windowed_context.make_current().unwrap() };

    println!("Pixel format of the window's GL context: {:?}", windowed_context.get_pixel_format());

    let gl = support::load(&windowed_context.context());

    el.run(move |event, _, control_flow| {
        // println!("{:?}", event);
        *control_flow = ControlFlow::Wait;

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
            Event::RedrawRequested(_) => {
                gl.draw_frame([0.0, 0.0, 0.0, 1.0]);
                windowed_context.swap_buffers().unwrap();
            }
            _ => (),
        }
    });
}


// Enumerate monitors and prompt user to choose one
fn prompt_for_monitor(el: &EventLoop<()>) -> MonitorHandle {
    for (num, monitor) in el.available_monitors().enumerate() {
        println!("Monitor #{}: {:?}", num, monitor.name());
    }

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
