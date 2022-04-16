use super::gl;

pub trait Id {
    fn id(&self) -> gl::types::GLuint;
}
