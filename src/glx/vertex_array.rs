use std::rc::Rc;
use super::support::gl;

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

