// extern crate gl;
// extern crate glutin;

use glutin::event::{Event, WindowEvent, VirtualKeyCode, ElementState, KeyboardInput};
use glutin::event_loop::{ControlFlow, EventLoop};
use glutin::window::{WindowBuilder, Fullscreen};
use glutin::ContextBuilder;
use glutin::monitor::{MonitorHandle};

mod support;

// use gl::types::*;
// use std::mem;
// use std::ptr;
// use std::str;
// use std::os::raw::c_void;
// use std::ffi::CString;
// use glutin::dpi::Size;
// 

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
                gl.draw_frame([1.0, 0.5, 0.7, 1.0]);

                // unsafe {
                //     gl::ClearColor(0.39, 0.58, 0.92, 1.0);
                //     gl::Clear(gl::COLOR_BUFFER_BIT);
                //     gl::UseProgram(shader_program);
                //     gl::BindVertexArray(vao);
                //     gl::DrawArrays(gl::TRIANGLES, 0, 3);
                // }

                windowed_context.swap_buffers().unwrap();
            }
            _ => (),
        }
    });

    // let gl_window = glutin::GlWindow::new(window, context, &events_loop).unwrap();
    // 
    // unsafe {
    //     gl_window.make_current().unwrap();
    //     gl::load_with(|symbol| gl_window.get_proc_address(symbol) as *const _);
    // }
    // 
    // let (shader_program, vao) = unsafe {
    //     // Vertex shader
    //     let vertex_shader = gl::CreateShader(gl::VERTEX_SHADER);
    //     let c_str_vert = CString::new(VERTEX_SHADER_SOURCE.as_bytes()).unwrap();
    //     gl::ShaderSource(vertex_shader, 1, &c_str_vert.as_ptr(), ptr::null());
    //     gl::CompileShader(vertex_shader);
    // // 
    //     // Fragment shader
    //     let fragment_shader = gl::CreateShader(gl::FRAGMENT_SHADER);
    //     let c_str_frag = CString::new(FRAGMENT_SHADER_SOURCE.as_bytes()).unwrap();
    //     gl::ShaderSource(fragment_shader, 1, &c_str_frag.as_ptr(), ptr::null());
    //     gl::CompileShader(fragment_shader);

    //     // Link Shaders
    //     let shader_program = gl::CreateProgram();
    //     gl::AttachShader(shader_program, vertex_shader);
    //     gl::AttachShader(shader_program, fragment_shader);
    //     gl::LinkProgram(shader_program);

    //     gl::DeleteShader(vertex_shader);
    //     gl::DeleteShader(fragment_shader);
    // 
    //     // Set up vao and vbos
    //     let vertices: [f32; 18] = [
    //         // left
    //         -0.5, -0.5, 0.0, 1.0, 0.0, 0.0,
    // 
    //         // right
    //         0.5, -0.5, 0.0, 0.0, 1.0, 0.0,
    // 
    //         // top
    //         0.0,  0.5, 0.0, 0.0, 0.0, 1.0
    //     ];
    // 
    //     let (mut vbo, mut vao) = (0, 0);
    //     gl::GenVertexArrays(1, &mut vao);
    //     gl::GenBuffers(1, &mut vbo);
    // 
    //     gl::BindVertexArray(vao);
    //     gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
    //     gl::BufferData(
    //         gl::ARRAY_BUFFER,
    //         (vertices.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
    //         &vertices[0] as *const f32 as *const c_void,
    //         gl::STATIC_DRAW,
    //     );
    // 
    //     gl::EnableVertexAttribArray(0);
    //     gl::VertexAttribPointer(
    //         0,
    //         3,
    //         gl::FLOAT,
    //         gl::FALSE,
    //         6 * mem::size_of::<GLfloat>() as GLsizei,
    //         ptr::null(),
    //     );
    // 
    //     gl::EnableVertexAttribArray(1);
    //     gl::VertexAttribPointer(
    //         1,
    //         3,
    //         gl::FLOAT,
    //         gl::FALSE,
    //         6 * mem::size_of::<GLfloat>() as GLsizei,
    //         (3 * mem::size_of::<GLfloat>()) as *const c_void
    //     );
    // 
    // 
    //     gl::BindBuffer(gl::ARRAY_BUFFER, 0);
    //     gl::BindVertexArray(0);
    // 
    //     // Wireframe
    //     // gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE);
    // 
    //     (shader_program, vao)
    // };
    // 
}


// Enumerate monitors and prompt user to choose one
fn prompt_for_monitor(el: &EventLoop<()>) -> MonitorHandle {
    for (num, monitor) in el.available_monitors().enumerate() {
        println!("Monitor #{}: {:?}", num, monitor.name());
    }

    // print!("Please write the number of the monitor to use: ");
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
