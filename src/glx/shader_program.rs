use super::support::gl;
use anyhow::{anyhow, Result};
use std::ffi::{CStr, CString};
use std::rc::Rc;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ShaderError {
    #[error("Shader compilation error: {0}")]
    Compilation(String),
    #[error("Shader linking error: {0}")]
    Linking(String),
    #[error("Shader lookup error: {0}")]
    Lookup(String),
}

pub struct ShaderProgram {
    program_id: gl::types::GLuint,
    pub(super) gl: Rc<gl::Gl>,
}

impl ShaderProgram {
    pub fn new(
        gl: &Rc<gl::Gl>,
        vertex_shader_src: &'static [u8],
        fragment_shader_src: &'static [u8],
    ) -> Result<ShaderProgram> {
        unsafe {
            let vertex_shader = compile_shader(&gl, vertex_shader_src, gl::VERTEX_SHADER)?;
            let fragment_shader = compile_shader(&gl, fragment_shader_src, gl::FRAGMENT_SHADER)?;
            let program = link_program(&gl, vertex_shader, fragment_shader)?;
            gl.DeleteShader(vertex_shader);
            gl.DeleteShader(fragment_shader);
            Ok(ShaderProgram {
                program_id: program,
                gl: gl.clone(),
            })
        }
    }

    pub fn activate(&self) {
        unsafe {
            self.gl.UseProgram(self.program_id);
        }
    }

    pub(super) fn get_attrib_location(&self, name: &str) -> Result<gl::types::GLint> {
        let c_name = CString::new(name).unwrap();
        unsafe {
            let location = self.gl.GetAttribLocation(self.program_id, c_name.as_ptr());
            if location == -1 {
                Err(anyhow!(ShaderError::Lookup(format!(
                    "'couldn't find attribute named '{}'",
                    name
                ))))
            } else {
                Ok(location)
            }
        }
    }

    pub(super) fn get_uniform_location(&self, name: &str) -> Result<gl::types::GLint> {
        let c_name = CString::new(name).unwrap();
        unsafe {
            let location = self.gl.GetUniformLocation(self.program_id, c_name.as_ptr());
            if location == -1 {
                Err(anyhow!(ShaderError::Lookup(format!(
                    "'couldn't find uniform named '{}'",
                    name
                ))))
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

unsafe fn compile_shader(
    gl: &gl::Gl,
    src: &'static [u8],
    shader_type: gl::types::GLenum,
) -> Result<gl::types::GLuint, ShaderError> {
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
        let message = CStr::from_ptr(info_log.as_ptr())
            .to_str()
            .expect("ShaderInfoLog not valid");
        Err(ShaderError::Compilation(message.into()))
    } else {
        Ok(shader)
    }
}

unsafe fn link_program(
    gl: &gl::Gl,
    vertex_shader: gl::types::GLuint,
    fragment_shader: gl::types::GLuint,
) -> Result<gl::types::GLuint, ShaderError> {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shader_error_should_explains_the_reason() {
        let message = "Cannot find it";
        let result: Result<()> = Err(anyhow!(ShaderError::Lookup(message.into())));

        assert!(result.is_err());
        if let Err(err) = result {
            assert_eq!(
                format!("{}", err),
                format!("Shader lookup error: {}", message)
            );
        }
    }
}
