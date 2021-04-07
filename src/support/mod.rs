use cgmath::prelude::*;

use std::ffi::CStr;
use cgmath::Matrix4;

#[macro_use]
mod macros;

pub mod gl {
    pub use self::Gles2 as Gl;
    include!(concat!(env!("OUT_DIR"), "/gl_bindings.rs"));
}

pub struct Gl {
    pub(crate) gl: gl::Gl,
    _program: gl::types::GLuint,
    mvp_attrib: gl::types::GLint,
    _vertex_array: gl::types::GLuint,
}

pub fn load(gl_context: &glutin::Context<glutin::PossiblyCurrent>) -> Gl {
    let gl = gl::Gl::load_with(|ptr| gl_context.get_proc_address(ptr) as *const _);

    let version = unsafe {
        let data = CStr::from_ptr(gl.GetString(gl::VERSION) as *const _).to_bytes().to_vec();
        String::from_utf8(data).unwrap()
    };

    println!("OpenGL version {}", version);

    let (program, mvp_attrib, vertex_array) =
        unsafe {
            let vs = gl.CreateShader(gl::VERTEX_SHADER);
            gl.ShaderSource(vs, 1, [VS_SRC.as_ptr() as *const _].as_ptr(), std::ptr::null());
            gl.CompileShader(vs);
            check_compile!(gl, vs, "Vertex Shader");

            let fs = gl.CreateShader(gl::FRAGMENT_SHADER);
            gl.ShaderSource(fs, 1, [FS_SRC.as_ptr() as *const _].as_ptr(), std::ptr::null());
            gl.CompileShader(fs);
            check_compile!(gl, fs, "Fragment Shader");

            let program = gl.CreateProgram();
            gl.AttachShader(program, vs);
            gl.AttachShader(program, fs);
            gl.LinkProgram(program);
            check_link!(gl, program, "Program");

            gl.DeleteShader(vs);
            gl.DeleteShader(fs);
            gl.UseProgram(program);

            // https://learnopengl.com/Getting-started/Hello-Triangle
            // https://github.com/bwasty/learn-opengl-rs/tree/master/src/_1_getting_started (warning out-of-date)
            let mut vbo = std::mem::zeroed();
            gl.GenBuffers(1, &mut vbo);
            gl.BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl.BufferData(
                gl::ARRAY_BUFFER,
                (VERTEX_DATA.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr,
                VERTEX_DATA.as_ptr() as *const _,
                gl::STATIC_DRAW,
            );

            let mut vao = std::mem::zeroed();
            if gl.BindVertexArray.is_loaded() {
                gl.GenVertexArrays(1, &mut vao);
                gl.BindVertexArray(vao);
            }

            let mvp_attrib = gl.GetUniformLocation(program, b"MVP\0".as_ptr() as *const _);
            let pos_attrib = gl.GetAttribLocation(program, b"vPos\0".as_ptr() as *const _);
            let color_attrib = gl.GetAttribLocation(program, b"vCol\0".as_ptr() as *const _);
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
            gl.EnableVertexAttribArray(pos_attrib as gl::types::GLuint);
            gl.EnableVertexAttribArray(color_attrib as gl::types::GLuint);

            (program, mvp_attrib, vao)
        };

    Gl { gl, _program: program, mvp_attrib, _vertex_array: vertex_array }
}

impl Gl {
    pub fn draw_frame(&self, t: f32, ratio: f32, color: [f32; 4]) {
        unsafe {
            let m = Matrix4::from_angle_z(cgmath::Rad(t));
            let p = cgmath::ortho(-ratio, ratio, -1., 1., 1., -1.);
            let mvp = p * m;

            self.gl.ClearColor(color[0], color[1], color[2], color[3]);
            self.gl.UniformMatrix4fv(self.mvp_attrib, 1, gl::FALSE, mvp.as_ptr() as *const f32);
            self.gl.Clear(gl::COLOR_BUFFER_BIT);

            // self.gl.UseProgram(self._program);
            // self.gl.BindVertexArray(self._vertex_array);

            self.gl.DrawArrays(gl::TRIANGLES, 0, 3);
        }
    }
}

#[rustfmt::skip]
static VERTEX_DATA: [f32; 15] = [
    // (position 2d + color 3d pack)
    -0.5, -0.5, 1.0, 0.0, 0.0, // left 
    0.0, 0.5, 0.0, 1.0, 0.0, // left
    0.5, -0.5, 0.0, 0.0, 1.0, // top
];

// Target Vertex Shader
const VS_SRC: &'static [u8] = b"
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

const FS_SRC: &'static [u8] = b"
#version 330 core

in vec3 color;
out vec4 fragColor;
void main() {
    // Set the fragment color to the color passed from the vertex shader
    fragColor = vec4(color, 1.0);
}
\0";
