use std::rc::Rc;
use super::support::gl;
use crate::glx::shader_program::ShaderProgram;
use super::buffer::Buffer;
use super::vertex_array::VertexArray;
use std::collections::HashMap;
use anyhow::{anyhow, Result};

pub struct ProgramUnit {
    program: ShaderProgram,
    vbo: Buffer,
    pub vao: VertexArray,
    uniforms: HashMap<&'static str, gl::types::GLint>,
    attributes: HashMap<&'static str, gl::types::GLint>,
}

impl ProgramUnit {
    pub fn new(gl: &Rc<gl::Gl>, vertex_shader_src: &'static [u8], fragment_shader_src: &'static [u8]) -> Result<Self> {
        Ok(ProgramUnit {
            program: ShaderProgram::new(&gl, vertex_shader_src, fragment_shader_src)?,
            vbo: Buffer::new(gl.clone()),
            vao: VertexArray::new(gl.clone()),
            uniforms: HashMap::default(),
            attributes: HashMap::default(),
        })
    }
    
    pub fn gl(&self) -> Rc<gl::Gl> {
        self.program.gl.clone()
    }
    
    pub fn prepare(self: &Self) {
        self.vao.bind(); // not sure about these bind before activate (empirical)
        self.vbo.bind(gl::ARRAY_BUFFER);
        self.program.activate();
    }

    pub fn activate(self: &Self) {
        // self.vao.bind(); 
        self.vbo.bind(gl::ARRAY_BUFFER); // not sure about these bind before activate (empirical)
        self.program.activate();
        unsafe { self.program.gl.BindVertexArray(self.vao.vertex_array_id); }
    }

    pub fn add_uniform(&mut self, name: &'static str) -> Result<gl::types::GLint> {
        let id = self.program.get_uniform_location(name)?;
        self.uniforms.insert(name, id);
        Ok(id)
    }

    pub fn add_attribute(&mut self, name: &'static str) -> Result<gl::types::GLint> {
        let id = self.program.get_attrib_location(name)?;
        self.attributes.insert(name, id);
        unsafe { self.program.gl.EnableVertexAttribArray(id as gl::types::GLuint); }
        Ok(id)
    }

    pub fn get_uniform(&self, name: &'static str) -> Result<gl::types::GLint> {
        match self.uniforms.get(name) {
            Some(id) => Ok(*id),
            None => Err(anyhow!(format!("Uniform '{}' not found", name)))
        }
    }

    pub fn get_attribute(&self, name: &'static str) -> Result<gl::types::GLint> {
        match self.attributes.get(name) {
            Some(id) => Ok(*id),
            None => Err(anyhow!(format!("Attribute '{}' not found", name)))
        }
    }
}
