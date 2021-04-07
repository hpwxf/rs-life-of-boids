use cgmath::{Matrix, Matrix4};

use crate::glx::{ShaderProgram, VertexArray, Buffer};

// // Shader sources
// static VS_SRC: &'static [u8] = b"
//     #version 330 core
//     layout (location = 0) in vec2 position;
//     layout (location = 1) in vec2 velocity;
// 
//     uniform mat3 transform;
//     uniform float pointSize;
//     uniform float maxSpeedSquared;
// 
//     out vec4 pointColor;
// 
//     float two_pi = 6.2831853072;
// 
//     vec3 rgb_from_hsb(in vec3 c){
//         vec3 rgb = clamp(abs(mod(c.x*6.0+vec3(0.0,4.0,2.0),
//                                  6.0)-3.0)-1.0,
//                          0.0,
//                          1.0 );
//         rgb = rgb*rgb*(3.0-2.0*rgb);
//         return c.z * mix(vec3(1.0), rgb, c.y);
//     }
// 
//     float mag_2 = pow(velocity.x, 2) + pow(velocity.y, 2);
// 
//     float a = atan(velocity.y, velocity.x);
//     void main() {
//         pointColor = vec4(rgb_from_hsb(vec3(a/two_pi, 1 - (mag_2 / maxSpeedSquared), 1.0)), 1.0);
//         gl_PointSize = pointSize;
//         gl_Position = vec4(transform * vec3(position, 1.0), 1.0);
//     }\0";
// 
// static FS_SRC: &'static [u8] = b"
//     #version 330 core
//     out vec4 frag_colour;
// 
//     in vec4 pointColor;
// 
//     void main() {
//         frag_colour = pointColor;
//     }\0";


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


//TODO: Handle resizing of screen
//TODO: How to run at different resolutions

pub struct RendererConfig {
    pub width: f32,
    pub height: f32,
    pub boid_size: f32, // TODO move out
    // pub max_speed: f32, // TODO move out
}

pub struct Renderer {
    pub gl: Rc<gl::Gl>,
    // transform: Matrix3<f32>,
    // boid_size: f32,
    // max_speed: f32,
    program: ShaderProgram,
    vbo: Buffer,
    vao: VertexArray,
    mvp_attrib: gl::types::GLint,
}

use crate::support::gl;
use std::rc::Rc;

impl Renderer {
    pub fn new(gl: gl::Gl, config: RendererConfig) -> Renderer {
        let gl = Rc::new(gl);

        let program = ShaderProgram::new(&gl, &VS_SRC, &FS_SRC)
            .expect("Error while build shader program");

        Renderer {
            // transform: glx::vtx_transform_2d(config.width, config.height),
            // boid_size: config.boid_size,
            // max_speed: config.max_speed,
            gl: gl.clone(),
            program,
            vbo: Buffer::new(gl.clone()),
            vao: VertexArray::new(gl.clone()),
            mvp_attrib: 0,
        }
    }

    pub fn initialize(&mut self) {
        let gl = self.gl.clone();

        self.vao.bind();
        self.vbo.bind(gl::ARRAY_BUFFER);
        self.program.activate();
        self.vbo.bind(gl::ARRAY_BUFFER);
        unsafe {
            gl.BufferData(
                gl::ARRAY_BUFFER,
                (VERTEX_DATA.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr,
                VERTEX_DATA.as_ptr() as *const _,
                gl::STATIC_DRAW,
            );
        }
        self.vao.bind();

        
        let (mvp_attrib) =
            unsafe {
                let program_id = self.program.program_id;
                let mvp_attrib = gl.GetUniformLocation(program_id, b"MVP\0".as_ptr() as *const _);
                let pos_attrib = gl.GetAttribLocation(program_id, b"vPos\0".as_ptr() as *const _);
                let color_attrib = gl.GetAttribLocation(program_id, b"vCol\0".as_ptr() as *const _);
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

                (mvp_attrib)
            };

        self.mvp_attrib = mvp_attrib;
    }


    pub fn init_pipeline(&self) {
        unsafe {
    //         // Set the tranform uniform
    //         let trans_loc = self
    //             .program
    //             .get_uniform_location("transform")
    //             .expect("Could not find uniform");
    //         gl::UniformMatrix3fv(trans_loc, 1, gl::FALSE, self.transform.as_ptr());
    // 
    //         // Set the point size
    //         let size_loc = self
    //             .program
    //             .get_uniform_location("pointSize")
    //             .expect("Could not find uniform");
    //         gl::Uniform1f(size_loc, self.boid_size as GLfloat);
    // 
    //         // Set max speed
    //         let max_speed_loc = self
    //             .program
    //             .get_uniform_location("maxSpeedSquared")
    //             .expect("Could not find uniform");
    //         gl::Uniform1f(max_speed_loc, self.max_speed.powi(2) as GLfloat);
    // 
    //         // Specify the layout of the vertex data
    //         let pos_loc = self
    //             .program
    //             .get_atrib_location("position")
    //             .expect("could not find position");
    //         gl::EnableVertexAttribArray(pos_loc);
    //         gl::VertexAttribPointer(
    //             pos_loc,
    //             2,
    //             gl::FLOAT,
    //             gl::FALSE,
    //             mem::size_of::<Boid>() as GLsizei,
    //             ptr::null(),
    //         );
    // 
    //         let vel_loc = self
    //             .program
    //             .get_atrib_location("velocity")
    //             .expect("could not find velocity");
    //         gl::EnableVertexAttribArray(vel_loc);
    //         gl::VertexAttribPointer(
    //             vel_loc,
    //             2,
    //             gl::FLOAT,
    //             gl::FALSE,
    //             mem::size_of::<Boid>() as GLsizei,
    //             mem::size_of::<Point2<f32>>() as *const GLvoid,
    //         );
    // 
    //         // Allow shader to specify point size
    //         gl::Enable(gl::PROGRAM_POINT_SIZE);
        }
    }
    // 
    // pub fn render(&self, boids: &[Boid]) {
    //     glx::clear_screen(0.1, 0.1, 0.1);
    //     unsafe {
    //         // This _should_ implement buffer orphaning
    //         gl::BufferData(gl::ARRAY_BUFFER, 0, ptr::null(), gl::STREAM_DRAW);
    // 
    //         gl::BufferData(
    //             gl::ARRAY_BUFFER,
    //             (boids.len() * mem::size_of::<Boid>()) as GLsizeiptr,
    //             boids.as_ptr() as *const _,
    //             gl::STREAM_DRAW,
    //         );
    // 
    //         gl::DrawArrays(gl::POINTS, 0, boids.len() as i32);
    //     }
    // }

    pub fn draw_frame(&self, t: f32, ratio: f32, color: [f32; 4]) {
        unsafe {
            let m = Matrix4::from_angle_z(cgmath::Rad(t));
            let p = cgmath::ortho(-ratio, ratio, -1., 1., 1., -1.);
            let mvp = p * m;

            self.gl.ClearColor(color[0], color[1], color[2], color[3]);
            self.gl.UniformMatrix4fv(self.mvp_attrib, 1, gl::FALSE, mvp.as_ptr() as *const f32);
            self.gl.Clear(gl::COLOR_BUFFER_BIT);

            self.gl.UseProgram(self.program.program_id);
            self.gl.BindVertexArray(self.vao.vertex_array_id);

            self.gl.DrawArrays(gl::TRIANGLES, 0, 3);
        }
    }
}
