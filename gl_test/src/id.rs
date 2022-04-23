use super::gl_support::gl;

pub trait Id {
    fn id(&self) -> gl::types::GLuint;
}
