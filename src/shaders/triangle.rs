#[rustfmt::skip]
pub static VERTEX_DATA: [f32; 15] = [
    // (position 2d + color 3d pack)
    0.0, 0.0, 1.0, 0.0, 0.0, // left 
    0.0, 1.0, 0.0, 1.0, 0.0, // right
    1.0, 0.0, 0.0, 0.0, 1.0, // top
];

// Target Vertex Shader
pub const VS_SRC: &'static [u8] = b"
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

pub const FS_SRC: &'static [u8] = b"
#version 330 core

in vec3 color;
out vec4 fragColor;
void main() {
    // Set the fragment color to the color passed from the vertex shader
    fragColor = vec4(color, 1.0);
}
\0";
