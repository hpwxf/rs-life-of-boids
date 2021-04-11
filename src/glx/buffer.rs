use super::support::gl;
use std::rc::Rc;

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
        Buffer {
            buffer_id,
            gl,
        }
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
