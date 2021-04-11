// Glx = Open GL extras (aka helper functions)

// use cgmath::Matrix3;
// use gl::{self, types::*};
// use std::{
//     error,
//     ffi::{CStr, CString},
//     fmt, ptr,
// };
// 
// pub fn get_gl_extensions() -> Vec<String> {
//     let mut results = vec![];
//     for i in 0..get_gl_int(gl::NUM_EXTENSIONS) {
//         results.push(get_gl_stri(gl::EXTENSIONS, i as u32));
//     }
//     results
// }
// 
// pub fn get_gl_int(name: GLenum) -> i32 {
//     let mut i = 0;
//     unsafe {
//         gl::GetIntegerv(name, &mut i);
//     }
//     i
// }
// 
// pub fn get_gl_str(name: GLenum) -> String {
//     unsafe { read_gl_str(gl::GetString(name)) }
// }
// 
// pub fn get_gl_stri(name: GLenum, i: GLuint) -> String {
//     unsafe { read_gl_str(gl::GetStringi(name, i)) }
// }
// 
// unsafe fn read_gl_str(ptr: *const u8) -> String {
//     CStr::from_ptr(ptr as *const _)
//         .to_str()
//         .expect("OpenGL returned invalid utf8")
//         .to_owned()
// }
// 
// pub fn vtx_transform_2d(width: f32, height: f32) -> Matrix3<f32> {
//     Matrix3::new(2. / width, 0., 0., 0., -2. / height, 0., -1., 1., 1.)
// }
// 
// pub fn clear_screen(r: GLfloat, g: GLfloat, b: GLfloat) {
//     unsafe {
//         gl::ClearColor(r, g, b, 1.0);
//         gl::Clear(gl::COLOR_BUFFER_BIT);
//     }
// }
// 

// private sub-modules
mod buffer;
mod support;
mod vertex_array;
mod shader_program;
mod program_unit;
mod window;

// local use
use std::rc::Rc;
use glutin::window::Window;
use std::path::{Path, PathBuf};
use glutin::dpi::PhysicalSize;
use anyhow::Result;

// re-export
pub use program_unit::ProgramUnit;
pub use support::gl;
pub use window::WindowSizeInfo;
pub use window::get_window_size_info;
pub use window::gl_init;

pub fn save_image(gl: Rc<gl::Gl>, filepath: &PathBuf, window: &Window) {
    let PhysicalSize { width, height } = window.inner_size();
    let nr_channels = 3;
    let mut stride = nr_channels * width;
    stride += if stride % 4 != 0 { 4 - stride % 4 } else { 0 };
    let buffer_size = stride * height;
    let mut data = Vec::<u8>::with_capacity(buffer_size as usize);
    data.resize(data.capacity(), 0);

    unsafe {
        gl.PixelStorei(gl::PACK_ALIGNMENT, 4);
        gl.ReadBuffer(gl::FRONT);
        gl.ReadPixels(0, 0, width as i32, height as i32, gl::RGB, gl::UNSIGNED_BYTE, data.as_ptr() as *mut _);
    }

    let mut img = image::RgbImage::new(width, height);
    for y in 0..height {
        let mut pos = (y * stride) as usize;
        for x in 0..width {
            let red = data[pos];
            pos += 1;
            let green = data[pos];
            pos += 1;
            let blue = data[pos];
            pos += 1;
            *img.get_pixel_mut(x, height - 1 - y) = image::Rgb([red, green, blue]);
        }
    }

    img.save(filepath).unwrap();
}