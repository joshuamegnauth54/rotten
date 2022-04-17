use super::gl_support::gl::types::{GLenum, GLint, GLuint};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum GlError {
    #[error("shader compilation failed with {0}")]
    Shader(String),
    #[error("creating the shader program failed with {0}")]
    ShaderProgram(String),
}
