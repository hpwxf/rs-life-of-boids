use super::support::gl;
use std::rc::Rc;

pub struct VertexArray {
    pub(super) vertex_array_id: gl::types::GLuint,
    gl: Rc<gl::Gl>,
}

impl VertexArray {
    pub fn new(gl: Rc<gl::Gl>) -> VertexArray {
        let mut vertex_array_id = 0;
        unsafe {
            gl.GenVertexArrays(1, &mut vertex_array_id);
        }
        VertexArray {
            vertex_array_id,
            gl,
        }
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
