use std::rc::Rc;
use super::support::gl;
use crate::glx::shader_program::{ShaderProgram, ShaderError};
use super::buffer::Buffer;
use super::vertex_array::VertexArray;
use std::collections::HashMap;
use anyhow::Result;

pub struct ProgramUnit {
    pub program: ShaderProgram,
    vbo: Buffer,
    pub vao: VertexArray,
    pub uniforms: HashMap<&'static str, gl::types::GLint>,
    pub attributes: HashMap<&'static str, gl::types::GLint>,
}

impl ProgramUnit {
    pub fn new(gl: &Rc<gl::Gl>, vertex_shader_src: &'static [u8], fragment_shader_src: &'static [u8]) -> Result<Self, ShaderError> {
        Ok(ProgramUnit {
            program: ShaderProgram::new(&gl, vertex_shader_src, fragment_shader_src)?,
            vbo: Buffer::new(gl.clone()),
            vao: VertexArray::new(gl.clone()),
            uniforms: HashMap::default(),
            attributes: HashMap::default(),
        })
    }

    pub fn activate(self: &Self) {
        self.vao.bind(); // not sure about these bind before activate (empirical)
        self.vbo.bind(gl::ARRAY_BUFFER);
        self.program.activate();
    }

    pub fn add_uniform(&self, p0: &str) -> Result<()>{
        todo!();
        Ok(())
    }
}
