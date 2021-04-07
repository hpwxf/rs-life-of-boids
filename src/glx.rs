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
#[derive(Debug)]
pub enum ShaderError {
    Compilation(String),
    Linking(String),
    Lookup(String),
    Undef,
}
// 
// impl fmt::Display for ShaderError {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         match *self {
//             ShaderError::Compilation(ref err) => write!(f, "Shader compilation error, {}", err),
//             ShaderError::Linking(ref err) => write!(f, "Shader linking error, {}", err),
//             ShaderError::Lookup(ref err) => write!(f, "Shader lookup error, {}", err),
//         }
//     }
// }
// 
// impl error::Error for ShaderError {
//     fn description(&self) -> &str {
//         match *self {
//             ShaderError::Compilation(ref err) => err,
//             ShaderError::Linking(ref err) => err,
//             ShaderError::Lookup(ref err) => err,
//         }
//     }
// 
//     fn cause(&self) -> Option<&dyn error::Error> {
//         None
//     }
// }
// 
// pub struct VertexArray {
//     vertex_array_id: GLuint,
// }
// 
// impl VertexArray {
//     pub fn new() -> VertexArray {
//         let mut vertex_array_id = 0;
//         unsafe {
//             gl::GenVertexArrays(1, &mut vertex_array_id);
//         }
//         VertexArray { vertex_array_id }
//     }
// 
//     pub fn bind(&self) {
//         unsafe {
//             gl::BindVertexArray(self.vertex_array_id);
//         }
//     }
// }
// 
// impl Drop for VertexArray {
//     fn drop(&mut self) {
//         unsafe {
//             gl::DeleteVertexArrays(1, &self.vertex_array_id);
//         }
//     }
// }
// 
// pub struct Buffer {
//     buffer_id: GLuint,
// }
// 
// impl Buffer {
//     pub fn new() -> Buffer {
//         let mut buffer_id = 0;
//         unsafe {
//             gl::GenBuffers(1, &mut buffer_id);
//         }
//         Buffer { buffer_id }
//     }
// 
//     pub fn bind(&self, target: GLenum) {
//         unsafe {
//             gl::BindBuffer(target, self.buffer_id);
//         }
//     }
// }
// 
// impl Drop for Buffer {
//     fn drop(&mut self) {
//         unsafe {
//             gl::DeleteBuffers(1, &self.buffer_id);
//         }
//     }
// }

use crate::support::gl;
use crate::support::Gl;
use std::ffi::{CString, CStr};

pub struct ShaderProgram {
    program_id: gl::types::GLuint,
}

impl ShaderProgram {
    // pub fn new(vrtx_src: &str, frag_src: &str) -> Result<ShaderProgram, ShaderError> {
    //     unsafe {
    //         let vrtx_shader = compile_shader(vrtx_src, gl::VERTEX_SHADER)?;
    //         let frag_shader = compile_shader(frag_src, gl::FRAGMENT_SHADER)?;
    //         let program_id = link_program(vrtx_shader, frag_shader)?;
    //         gl::DeleteShader(vrtx_shader);
    //         gl::DeleteShader(frag_shader);
    //         let program = ShaderProgram { program_id };
    //         Ok(program)
    //     }
    // }
    // 
    // pub fn activate(&self) {
    //     unsafe {
    //         gl::UseProgram(self.program_id);
    //     }
    // }
    // 
    // pub fn get_atrib_location(&self, name: &str) -> Result<GLuint, ShaderError> {
    //     let c_name = CString::new(name).unwrap();
    //     unsafe {
    //         let location = gl::GetAttribLocation(self.program_id, c_name.as_ptr());
    //         if location == -1 {
    //             Err(ShaderError::Lookup(format!(
    //                 "'couldn't find attribute named '{}'",
    //                 name
    //             )))
    //         } else {
    //             Ok(location as GLuint)
    //         }
    //     }
    // }
    // 
    // pub fn get_uniform_location(&self, name: &str) -> Result<GLint, ShaderError> {
    //     let c_name = CString::new(name).unwrap();
    //     unsafe {
    //         let location = gl::GetUniformLocation(self.program_id, c_name.as_ptr());
    //         if location == -1 {
    //             Err(ShaderError::Lookup(format!(
    //                 "'couldn't find uniform named '{}'",
    //                 name
    //             )))
    //         } else {
    //             Ok(location)
    //         }
    //     }
    // }
}

// impl Drop for ShaderProgram {
//     fn drop(&mut self) {
//         unsafe {
//             gl::DeleteShader(self.program_id);
//         }
//     }
// }
// 
unsafe fn compile_shader(src: &str, gl: &gl::Gl, shader_type: gl::types::GLenum) -> Result<gl::types::GLuint, ShaderError> {
    let shader = gl.CreateShader(shader_type);

    // Attempt to compile shader
    let c_str = CString::new(src.as_bytes()).unwrap();
    gl.ShaderSource(shader, 1, &c_str.as_ptr(), std::ptr::null());
    gl.CompileShader(shader);

    // Ckeck compilation errors
    let mut success = i32::from(gl::FALSE);
    gl.GetShaderiv(shader, gl::COMPILE_STATUS, &mut success);
    if success != i32::from(gl::TRUE) {
        let mut len = 0;
        gl.GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut len);
        let mut info_log = Vec::with_capacity(len as usize);
        info_log.set_len(len as usize - 1); // -1 to skip trialing null character
        gl.GetShaderInfoLog(
            shader,
            len,
            std::ptr::null_mut(),
            info_log.as_mut_ptr(), // as *mut gl::types::GLchar : FIXME useless ?
        );
        let message = CStr::from_ptr(info_log.as_ptr()).to_str().expect("ShaderInfoLog not valid");
        Err(ShaderError::Compilation(message.into()))
    } else {
        Ok(shader)
    }
}

unsafe fn link_program(gl: &gl::Gl, vertex_shader: gl::types::GLuint, fragment_shader: gl::types::GLuint) -> Result<gl::types::GLuint, ShaderError> {
    let program = gl.CreateProgram();
    gl.AttachShader(program, vertex_shader);
    gl.AttachShader(program, fragment_shader);
    gl.LinkProgram(program);

    // Check link errors
    let mut success = i32::from(gl::FALSE);
    gl.GetProgramiv(program, gl::LINK_STATUS, &mut success);
    if success != i32::from(gl::TRUE) {
        let mut len = 0;
        gl.GetProgramiv(program, gl::INFO_LOG_LENGTH, &mut len);
        let mut info_log = Vec::with_capacity(len as usize);
        info_log.set_len(len as usize - 1); // -1 to skip trialing null character
        gl.GetProgramInfoLog(
        program,
        len,
        std::ptr::null_mut(),
        info_log.as_mut_ptr() // as *mut gl::types::GLchar : FIXME useless ?
        );
        let message = CStr::from_ptr(info_log.as_ptr()).to_str().unwrap();
        Err(ShaderError::Linking(message.into()))
    } else {
        Ok(program)
    } 
}
