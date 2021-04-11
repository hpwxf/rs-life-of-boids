use std::rc::Rc;

use anyhow::Result;
use cgmath::{Matrix, Matrix3, Point2, Vector2};

use crate::glx::gl;
use crate::glx::{vertex_transform_2d, ProgramUnit, WindowSizeInfo};

pub type Position = Point2<f32>;
pub type Velocity = Vector2<f32>;

pub struct Point {
    pub(crate) position: Position,
    pub(crate) velocity: Velocity,
}

pub struct PointsRenderProgram {
    program: ProgramUnit,
    transform: Matrix3<f32>,
    point_size: f32,
    max_speed: f32,
}

impl PointsRenderProgram {
    pub fn new(gl: Rc<gl::Gl>, size: WindowSizeInfo) -> Result<Self> {
        Ok(PointsRenderProgram {
            program: ProgramUnit::new(&gl, &VS_SRC, &FS_SRC)?,
            transform: vertex_transform_2d(size.width as f32, size.height as f32),
            point_size: 3.0,
            max_speed: 10.0,
        })
    }

    pub fn initialize(&mut self) -> Result<()> {
        let gl = self.program.gl();

        self.program.prepare();
        unsafe {
            self.program.add_uniform("transform")?;
            self.program.add_uniform("pointSize")?;
            self.program.add_uniform("maxSpeedSquared")?;
            // Specify the layout of the vertex data
            let pos_loc = self.program.add_attribute("position")?;
            gl.VertexAttribPointer(
                pos_loc as gl::types::GLuint,
                2,
                gl::FLOAT,
                gl::FALSE,
                std::mem::size_of::<Point>() as gl::types::GLsizei,
                std::ptr::null(), // or 0 as *const gl::types::GLvoid,
            );
            let vel_loc = self.program.add_attribute("velocity")?;
            gl.VertexAttribPointer(
                vel_loc as gl::types::GLuint,
                2,
                gl::FLOAT,
                gl::FALSE,
                std::mem::size_of::<Point>() as gl::types::GLsizei,
                std::mem::size_of::<Position>() as *const gl::types::GLvoid,
            );

            // Allow shader to specify point size
            gl.Enable(gl::PROGRAM_POINT_SIZE);
        }
        Ok(())
    }

    pub fn render(&self, points: &[Point]) -> Result<()> {
        let gl = self.program.gl();

        self.program.activate();
        unsafe {
            gl.UniformMatrix3fv(
                self.program.get_uniform("transform")?,
                1,
                gl::FALSE,
                self.transform.as_ptr(),
            );
            gl.Uniform1f(
                self.program.get_uniform("pointSize")?,
                self.point_size as gl::types::GLfloat,
            );
            gl.Uniform1f(
                self.program.get_uniform("maxSpeedSquared")?,
                self.max_speed.powi(2) as gl::types::GLfloat,
            );
            gl.BufferData(
                gl::ARRAY_BUFFER,
                (points.len() * std::mem::size_of::<Point>()) as gl::types::GLsizeiptr,
                points.as_ptr() as *const _,
                gl::STREAM_DRAW,
            );
            gl.DrawArrays(gl::POINTS, 0, points.len() as i32);
        }
        Ok(())
    }
}

// Shader sources
pub static VS_SRC: &[u8] = b"
    #version 330 core
    layout (location = 0) in vec2 position;
    layout (location = 1) in vec2 velocity;

    uniform mat3 transform;
    uniform float pointSize;
    uniform float maxSpeedSquared;

    out vec4 pointColor;

    float two_pi = 6.2831853072;

    vec3 rgb_from_hsb(in vec3 c){
        vec3 rgb = clamp(abs(mod(c.x*6.0+vec3(0.0,4.0,2.0),
                                 6.0)-3.0)-1.0,
                         0.0,
                         1.0 );
        rgb = rgb*rgb*(3.0-2.0*rgb);
        return c.z * mix(vec3(1.0), rgb, c.y);
    }

    float mag_2 = pow(velocity.x, 2) + pow(velocity.y, 2);

    float a = atan(velocity.y, velocity.x);
    void main() {
        // pointColor = vec4(rgb_from_hsb(vec3(a/two_pi, 1 - (mag_2 / maxSpeedSquared), 1.0)), 1.0);
        pointColor = vec4(mag_2 / maxSpeedSquared, mag_2 / maxSpeedSquared, mag_2 / maxSpeedSquared, 1.0);
        gl_PointSize = pointSize;
        gl_Position = vec4(transform * vec3(position, 1.0), 1.0);
    }\0";

pub static FS_SRC: &[u8] = b"
    #version 330 core
    out vec4 frag_colour;

    in vec4 pointColor;

    void main() {
        frag_colour = pointColor;
    }\0";
