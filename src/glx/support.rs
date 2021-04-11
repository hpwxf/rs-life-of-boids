#[allow(clippy::all)]

pub mod gl {
    pub use self::Gles2 as Gl;
    include!(concat!(env!("OUT_DIR"), "/gl_bindings.rs"));
    // TODO PR to add this constant
    pub(crate) const PROGRAM_POINT_SIZE: types::GLenum = 0x8642;
}
