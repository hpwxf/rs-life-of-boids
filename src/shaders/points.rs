use cgmath::{Point2, Vector2, Matrix3};

pub type Position = Point2<f32>;
pub type Velocity = Vector2<f32>;

pub struct Point {
    pub(crate) position: Position,
    pub(crate) velocity: Velocity,
}

pub fn vertex_transform_2d(width: f32, height: f32) -> Matrix3<f32> {
    Matrix3::new(2. / width, 0., 0., 0., -2. / height, 0., -1., 1., 1.)
}

// Shader sources
pub static VS_SRC: &'static [u8] = b"
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

pub static FS_SRC: &'static [u8] = b"
    #version 330 core
    out vec4 frag_colour;

    in vec4 pointColor;

    void main() {
        frag_colour = pointColor;
    }\0";
