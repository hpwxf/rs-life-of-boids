use std::ffi::CStr;
use std::rc::Rc;

use cgmath::Matrix3;
use glutin::window::Window;
use glutin::{ContextWrapper, PossiblyCurrent};

use super::support::gl;

#[derive(Debug, Copy, Clone)]
pub struct WindowSizeInfo {
    pub width: u32,
    pub height: u32,
    // hidpi_factor: f64,
}

#[derive(Debug)]
pub enum CustomError {}

pub fn gl_init(windowed_context: &ContextWrapper<PossiblyCurrent, Window>) -> gl::Gl {
    let gl_context = windowed_context.context();
    let gl = gl::Gl::load_with(|ptr| gl_context.get_proc_address(ptr) as *const _);

    let version = unsafe {
        let data = CStr::from_ptr(gl.GetString(gl::VERSION) as *const _)
            .to_bytes()
            .to_vec();
        String::from_utf8(data).unwrap()
    };

    println!("OpenGL version {}", version);
    gl
}

pub fn get_window_size_info(window: &Window) -> Result<WindowSizeInfo, CustomError> {
    // let hidpi_factor = window.get_hidpi_factor();
    // let logical_size = window
    //     .get_inner_size()
    //     .ok_or_else(|| CustomError::WindowIngo("Tried to get size of closed window".to_string()))?;
    // let physical_size = logical_size.to_physical(hidpi_factor);

    let physical_size = window.inner_size();

    Ok(WindowSizeInfo {
        width: physical_size.width,
        height: physical_size.height,
        // hidpi_factor,
    })
}

pub fn clear_screen(gl: &Rc<gl::Gl>, color: [f32; 4]) {
    unsafe {
        gl.ClearColor(color[0], color[1], color[2], color[3]);
        gl.Clear(gl::COLOR_BUFFER_BIT);
    }
}

pub fn vertex_transform_2d(width: f32, height: f32) -> Matrix3<f32> {
    Matrix3::new(2. / width, 0., 0., 0., -2. / height, 0., -1., 1., 1.)
}
