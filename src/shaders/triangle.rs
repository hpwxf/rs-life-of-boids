use crate::glx::gl;
use crate::glx::ProgramUnit;
use anyhow::Result;
use cgmath::{Matrix, Matrix4};
use std::rc::Rc;

pub struct TriangleRenderProgram {
    program: ProgramUnit,
}

impl TriangleRenderProgram {
    pub fn new(gl: Rc<gl::Gl>) -> Result<Self> {
        Ok(TriangleRenderProgram {
            program: ProgramUnit::new(&gl, &VS_SRC, &FS_SRC)?,
        })
    }

    pub fn initialize(&mut self) -> Result<()> {
        let gl = self.program.gl();

        self.program.prepare();
        self.program.add_uniform("MVP")?;
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

    pub fn render(&self, t: f32, ratio: f32) -> Result<()> {
        let gl = self.program.gl();

        let m = Matrix4::from_angle_z(cgmath::Rad(t));
        let p = cgmath::ortho(-ratio, ratio, -1., 1., 1., -1.);
        let mvp = p * m;

        self.program.prepare();
        unsafe {
            gl.UniformMatrix4fv(
                self.program.get_uniform("MVP")?,
                1,
                gl::FALSE,
                mvp.as_ptr() as *const f32,
            );
            gl.BufferData(
                gl::ARRAY_BUFFER,
                (crate::shaders::triangle::VERTEX_DATA.len() * std::mem::size_of::<f32>())
                    as gl::types::GLsizeiptr,
                crate::shaders::triangle::VERTEX_DATA.as_ptr() as *const _,
                gl::STATIC_DRAW,
            );
            gl.DrawArrays(gl::TRIANGLES, 0, 3);
        }
        Ok(())
    }
}

#[rustfmt::skip]
pub static VERTEX_DATA: [f32; 15] = [
    // (position 2d + color 3d pack)
    0.0, 0.0, 1.0, 0.0, 0.0, // left 
    0.0, 1.0, 0.0, 1.0, 0.0, // right
    1.0, 0.0, 0.0, 0.0, 1.0, // top
];

// Target Vertex Shader
pub const VS_SRC: &[u8] = b"
#version 330 core
uniform mat4 MVP;
in vec2 vPos;
in vec3 vCol; // Specify a vertex attribute for color
out vec3 color;
void main()
{
    // gl_Position = vec4(vPos.x, vPos.y, 0.0, 1.0);
    gl_Position = MVP * vec4(vPos, /* z */ 0.0, /* scale ? */ 1.0);
    // gl_PointSize = 100.0;
	color = vCol; // pass the color along to the fragment shader
}
\0";

pub const FS_SRC: &[u8] = b"
#version 330 core

in vec3 color;
out vec4 fragColor;
void main() {
    // Set the fragment color to the color passed from the vertex shader
    fragColor = vec4(color, 1.0);
}
\0";
