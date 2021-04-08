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

use std::ffi::{CStr, CString};
use std::rc::Rc;

use glutin::{ContextWrapper, PossiblyCurrent};
use glutin::window::Window;

use crate::support::gl;

#[derive(Debug)]
pub enum ShaderError {
    Compilation(String),
    Linking(String),
    Lookup(String)
}


impl std::fmt::Display for ShaderError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            ShaderError::Compilation(ref err) => write!(f, "Shader compilation error, {}", err),
            ShaderError::Linking(ref err) => write!(f, "Shader linking error, {}", err),
            ShaderError::Lookup(ref err) => write!(f, "Shader lookup error, {}", err),
        }
    }
}

impl std::error::Error for ShaderError {
    fn description(&self) -> &str {
        match *self {
            ShaderError::Compilation(ref err) => err,
            ShaderError::Linking(ref err) => err,
            ShaderError::Lookup(ref err) => err,
        }
    }

    fn cause(&self) -> Option<&dyn std::error::Error> {
        None
    }
}

pub struct VertexArray {
    pub vertex_array_id: gl::types::GLuint,
    gl: Rc<gl::Gl>,
}

impl VertexArray {
    pub fn new(gl: Rc<gl::Gl>) -> VertexArray {
        let mut vertex_array_id = 0;
        unsafe {
            gl.GenVertexArrays(1, &mut vertex_array_id);
        }
        VertexArray { vertex_array_id, gl: gl.clone() }
    }

    pub fn bind(&self) {
        unsafe {
            self.gl.BindVertexArray(self.vertex_array_id);
        }
    }
}

impl Drop for VertexArray {
    fn drop(&mut self) {
        unsafe {
            self.gl.DeleteVertexArrays(1, &self.vertex_array_id);
        }
    }
}

// https://learnopengl.com/Getting-started/Hello-Triangle
// https://github.com/bwasty/learn-opengl-rs/tree/master/src/_1_getting_started (warning out-of-date)
pub struct Buffer {
    buffer_id: gl::types::GLuint,
    gl: Rc<gl::Gl>,
}

impl Buffer {
    pub fn new(gl: Rc<gl::Gl>) -> Buffer {
        let mut buffer_id = 0;
        unsafe {
            gl.GenBuffers(1, &mut buffer_id);
        }
        Buffer { buffer_id, gl: gl.clone() }
    }

    pub fn bind(&self, target: gl::types::GLenum) {
        unsafe {
            self.gl.BindBuffer(target, self.buffer_id);
        }
    }
}

impl Drop for Buffer {
    fn drop(&mut self) {
        unsafe {
            self.gl.DeleteBuffers(1, &self.buffer_id);
        }
    }
}

pub struct ShaderProgram {
    pub(crate) program_id: gl::types::GLuint,
    pub(crate) gl: Rc<gl::Gl>,
}

impl ShaderProgram {
    pub fn new(gl: &Rc<gl::Gl>, vertex_shader_src: &'static [u8], fragment_shader_src: &'static [u8]) -> Result<ShaderProgram, ShaderError> {
        unsafe {
            let vertex_shader = compile_shader(&gl, vertex_shader_src, gl::VERTEX_SHADER)?;
            let fragment_shader = compile_shader(&gl, fragment_shader_src, gl::FRAGMENT_SHADER)?;
            let program = link_program(&gl, vertex_shader, fragment_shader)?;
            gl.DeleteShader(vertex_shader);
            gl.DeleteShader(fragment_shader);
            Ok(ShaderProgram { program_id: program, gl: gl.clone() })
        }
    }
    
    pub fn activate(&self) {
        unsafe {
            self.gl.UseProgram(self.program_id);
        }
    }
    
    pub fn get_attrib_location(&self, name: &str) -> Result<gl::types::GLint, ShaderError> {
        let c_name = CString::new(name).unwrap();
        unsafe {
            let location = self.gl.GetAttribLocation(self.program_id, c_name.as_ptr());
            if location == -1 {
                Err(ShaderError::Lookup(format!("'couldn't find attribute named '{}'", name)))
            } else {
                Ok(location)
            }
        }
    }
    
    pub fn get_uniform_location(&self, name: &str) -> Result<gl::types::GLint, ShaderError> {
        let c_name = CString::new(name).unwrap();
        unsafe {
            let location = self.gl.GetUniformLocation(self.program_id, c_name.as_ptr());
            if location == -1 {
                Err(ShaderError::Lookup(format!("'couldn't find uniform named '{}'", name)))
            } else {
                Ok(location)
            }
        }
    }
}

impl Drop for ShaderProgram {
    fn drop(&mut self) {
        unsafe {
            self.gl.DeleteShader(self.program_id);
        }
    }
}

unsafe fn compile_shader(gl: &gl::Gl, src: &'static [u8], shader_type: gl::types::GLenum) -> Result<gl::types::GLuint, ShaderError> {
    let shader = gl.CreateShader(shader_type);

    // Attempt to compile shader
    let c_str = [src.as_ptr() as *const _];
    gl.ShaderSource(shader, 1, c_str.as_ptr(), std::ptr::null());
    gl.CompileShader(shader);

    // Check compilation errors
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
            info_log.as_mut_ptr(), // as *mut gl::types::GLchar : FIXME useless ?
        );
        let message = CStr::from_ptr(info_log.as_ptr()).to_str().unwrap();
        Err(ShaderError::Linking(message.into()))
    } else {
        Ok(program)
    }
}

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
