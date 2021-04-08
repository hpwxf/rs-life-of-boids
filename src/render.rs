use std::rc::Rc;

use cgmath::{Matrix, Matrix4, Matrix3, SquareMatrix, Point2, Vector2};

use crate::glx::{Buffer, ShaderProgram, VertexArray, WindowSizeInfo};
use crate::support::gl;
use crate::WindowConfig;
use crate::shaders::points::{vertex_transform_2d, Point, Velocity};
use crate::shaders::triangle::VERTEX_DATA;

#[derive(PartialEq)]
enum Mode {
    Triangle,
    Points,
}

// const MODE: Mode = Mode::Triangle;
const MODE: Mode = Mode::Points;


pub struct RendererConfig {
    pub width: f32,
    pub height: f32,
}

pub struct Renderer {
    pub gl: Rc<gl::Gl>,
    program1: ShaderProgram,
    vbo1: Buffer,
    vao1: VertexArray,
    program2: ShaderProgram,
    vbo2: Buffer,
    vao2: VertexArray,
    mvp_attrib: gl::types::GLint,
    //
    transform: Matrix3<f32>,
    point_size: f32,
    max_speed: f32,
}

impl Renderer {
    pub fn new(gl: gl::Gl, config: RendererConfig) -> Renderer {
        let gl = Rc::new(gl);

        let program1 = ShaderProgram::new(&gl, &crate::shaders::triangle::VS_SRC, &crate::shaders::triangle::FS_SRC)
            .expect("Error while build shader program");
        let program2 = ShaderProgram::new(&gl, &crate::shaders::points::VS_SRC, &crate::shaders::points::FS_SRC)
            .expect("Error while build shader program");

        Renderer {
            gl: gl.clone(),
            // for triangle
            program1,
            vbo1: Buffer::new(gl.clone()),
            vao1: VertexArray::new(gl.clone()),
            mvp_attrib: 0,
            // for points
            program2,
            vbo2: Buffer::new(gl.clone()),
            vao2: VertexArray::new(gl.clone()),
            transform: vertex_transform_2d(config.width, config.height),
            point_size: 3.0,
            max_speed: 10.0,
        }
    }

    pub fn initialize(&mut self) {
        let gl = self.gl.clone();

        self.vao1.bind();
        self.vbo1.bind(gl::ARRAY_BUFFER);
        self.program1.activate();
        self.vbo1.bind(gl::ARRAY_BUFFER);
        // if MODE == Mode::Triangle {
            unsafe {
                gl.BufferData(
                    gl::ARRAY_BUFFER,
                    (VERTEX_DATA.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr,
                    VERTEX_DATA.as_ptr() as *const _,
                    gl::STATIC_DRAW,
                );
            }
        // }
        self.vao1.bind();

        // if MODE == Mode::Triangle {
            self.program1.activate();
            let mvp_attrib = self
                .program1
                .get_uniform_location("MVP")
                .expect("Could not find uniform");
            let pos_attrib = self
                .program1
                .get_attrib_location("vPos")
                .expect("Could not find vPos attribute");
            let color_attrib = self
                .program1
                .get_attrib_location("vCol")
                .expect("Could not find vCol attribute");

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

                // self.gl.UniformMatrix4fv(mvp_attrib, 1, gl::FALSE, mvp.as_ptr() as *const f32);

                gl.EnableVertexAttribArray(pos_attrib as gl::types::GLuint);
                gl.EnableVertexAttribArray(color_attrib as gl::types::GLuint);
            };
            self.mvp_attrib = mvp_attrib;
        // }

        // if MODE == Mode::Points {
            self.program2.activate();
            unsafe {
                // Set the transform uniform
                let trans_loc = self
                    .program2
                    .get_uniform_location("transform")
                    .expect("Could not find uniform");
                gl.UniformMatrix3fv(trans_loc, 1, gl::FALSE, self.transform.as_ptr());

                // Set the point size
                let size_loc = self
                    .program2
                    .get_uniform_location("pointSize")
                    .expect("Could not find uniform");
                gl.Uniform1f(size_loc, self.point_size as gl::types::GLfloat);

                // Set max speed
                let max_speed_loc = self
                    .program2
                    .get_uniform_location("maxSpeedSquared")
                    .expect("Could not find uniform");
                gl.Uniform1f(max_speed_loc, self.max_speed.powi(2) as gl::types::GLfloat);

                // Specify the layout of the vertex data
                let pos_loc = self
                    .program2
                    .get_attrib_location("position")
                    .expect("could not find position");
                gl.EnableVertexAttribArray(pos_loc as gl::types::GLuint);
                gl.VertexAttribPointer(
                    pos_loc as gl::types::GLuint,
                    2,
                    gl::FLOAT,
                    gl::FALSE,
                    std::mem::size_of::<Point>() as gl::types::GLsizei,
                    std::ptr::null(), // or std::mem::size_of::<Position>() as *const gl::types::GLvoid,
                );

                let vel_loc = self
                    .program2
                    .get_attrib_location("velocity")
                    .expect("could not find velocity");
                gl.EnableVertexAttribArray(vel_loc as gl::types::GLuint);
                gl.VertexAttribPointer(
                    vel_loc as gl::types::GLuint,
                    2,
                    gl::FLOAT,
                    gl::FALSE,
                    std::mem::size_of::<Point>() as gl::types::GLsizei,
                    std::mem::size_of::<Velocity>() as *const gl::types::GLvoid,
                );

                // Allow shader to specify point size
                gl.Enable(gl::PROGRAM_POINT_SIZE);
            }
        // }
    }

    pub fn render(&self, t: f32, ratio: f32, color: [f32; 4], window_info: &WindowSizeInfo, points: &[Point]) {
        unsafe {
            self.gl.ClearColor(color[0], color[1], color[2], color[3]);
            self.gl.Clear(gl::COLOR_BUFFER_BIT);
        }

        if MODE == Mode::Triangle {
            let m = Matrix4::from_angle_z(cgmath::Rad(t));
            let p = cgmath::ortho(-ratio, ratio, -1., 1., 1., -1.);
            let mvp = p * m;

            self.program1.activate();
            unsafe {
                self.gl.UniformMatrix4fv(self.mvp_attrib, 1, gl::FALSE, mvp.as_ptr() as *const f32);
                self.gl.BindVertexArray(self.vao1.vertex_array_id);
                self.gl.DrawArrays(gl::TRIANGLES, 0, 3);
            }
        }

        if MODE == Mode::Points {
            self.program2.activate();
            unsafe {
                // This _should_ implement buffer orphaning
                self.gl.BufferData(gl::ARRAY_BUFFER, 0, std::ptr::null(), gl::STREAM_DRAW);
                self.gl.BufferData(
                    gl::ARRAY_BUFFER,
                    (points.len() * std::mem::size_of::<Point>()) as gl::types::GLsizeiptr,
                    points.as_ptr() as *const _,
                    gl::STREAM_DRAW,
                );
                self.gl.DrawArrays(gl::POINTS, 0, points.len() as i32);
            }
        }
    }
}
