use crate::glx::{ProgramUnit, vertex_transform_2d};
use crate::glx::gl;
use anyhow::Result;
use std::rc::Rc;
use cgmath::{Matrix3, Matrix};

pub struct LinesRenderProgram {
    program: ProgramUnit,
}

impl LinesRenderProgram {
    pub fn new(gl: Rc<gl::Gl>) -> Result<Self> {
        Ok(LinesRenderProgram {
            program: ProgramUnit::new(&gl, &crate::shaders::lines::VS_SRC, &crate::shaders::lines::FS_SRC)?,
        })
    }

    pub fn initialize(&mut self) -> Result<()> {
        let gl = self.program.gl();

        self.program.prepare();
        self.program.add_uniform("transform")?;
        let pos_attrib = self.program.add_attribute("vPos")?;
        let color_attrib = self.program.add_attribute("vCol")?;

        unsafe {
            gl.VertexAttribPointer(
                pos_attrib as gl::types::GLuint,
                2,
                gl::FLOAT,
                gl::FALSE,
                5 * std::mem::size_of::<f32>() as gl::types::GLsizei,
                std::ptr::null(), // set offset in data
            );
            gl.VertexAttribPointer(
                color_attrib as gl::types::GLuint,
                3,
                gl::FLOAT,
                gl::FALSE,
                5 * std::mem::size_of::<f32>() as gl::types::GLsizei,
                (2 * std::mem::size_of::<f32>()) as *const () as *const _, // set offset in data
            );
        };
        Ok(())
    }

    pub fn render(&self, size: (u32, u32)) -> Result<()> {
        let gl = self.program.gl();

        let (width, height) = size;
        let transform: Matrix3<f32> = vertex_transform_2d(width as f32, height as f32);

        let mut vertex_data = Vec::<f32>::with_capacity(100); // need better size 
        vertex_data.extend_from_slice(&[(0 as f32) / 2.0, (size.1 as f32) / 2.0, 1.0, 1.0, 1.0]);
        vertex_data.extend_from_slice(&[(size.0 as f32), (size.1 as f32) / 2.0, 1.0, 1.0, 1.0]);
        vertex_data.extend_from_slice(&[(size.0 as f32) / 2.0, (0 as f32) / 2.0, 1.0, 1.0, 1.0]);
        vertex_data.extend_from_slice(&[(size.0 as f32) / 2.0, (size.1 as f32) / 2.0, 1.0, 1.0, 1.0]);

        self.program.activate();
        unsafe {
            gl.UniformMatrix3fv(self.program.get_uniform("transform")?, 1, gl::FALSE, transform.as_ptr());
            gl.BufferData(
                gl::ARRAY_BUFFER,
                (vertex_data.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr,
                vertex_data.as_ptr() as *const _,
                gl::STREAM_DRAW,
            );
            // https://docs.gl/gl3/glDrawArrays
            // https://www.khronos.org/opengl/wiki/Primitive (use gl::LINE_LOOP to make lines)
            gl.DrawArrays(gl::LINES, 0, (vertex_data.len() / 5) as i32);
        }
        Ok(())
    }
}


// Target Vertex Shader
const VS_SRC: &'static [u8] = b"
#version 330 core
uniform mat3 transform;
in vec2 vPos;
in vec3 vCol; // Specify a vertex attribute for color
out vec3 color;
void main()
{
    gl_Position = vec4(transform * vec3(vPos, 1.0), 1.0);
	color = vCol; // pass the color along to the fragment shader
}
\0";

const FS_SRC: &'static [u8] = b"
#version 330 core

in vec3 color;
out vec4 fragColor;
void main() {
    // Set the fragment color to the color passed from the vertex shader
    fragColor = vec4(color, 1.0);
}
\0";

