use super::support::gl;
use glutin::window::Window;
use glutin::{PossiblyCurrent, ContextWrapper};
use std::ffi::CStr;


pub struct WindowSizeInfo {
    pub width: f32,
    pub height: f32,
    // hidpi_factor: f64,
}

#[derive(Debug)]
pub enum CustomError {
    // WindowInfo(String)
}

pub fn get_window_size_info(window: &Window) -> Result<WindowSizeInfo, CustomError> {
    // let hidpi_factor = window.get_hidpi_factor();
    // let logical_size = window
    //     .get_inner_size()
    //     .ok_or_else(|| CustomError::WindowIngo("Tried to get size of closed window".to_string()))?;
    // let physical_size = logical_size.to_physical(hidpi_factor);

    let physical_size = window.inner_size();

    Ok(WindowSizeInfo {
        width: physical_size.width as f32,
        height: physical_size.height as f32,
        // hidpi_factor,
    })
}

pub fn gl_init(windowed_context: &ContextWrapper<PossiblyCurrent, Window>) -> gl::Gl {
    let gl_context = windowed_context.context();
    let gl = gl::Gl::load_with(|ptr| gl_context.get_proc_address(ptr) as *const _);

    let version = unsafe {
        let data = CStr::from_ptr(gl.GetString(gl::VERSION) as *const _).to_bytes().to_vec();
        String::from_utf8(data).unwrap()
    };

    println!("OpenGL version {}", version);
    gl
}

