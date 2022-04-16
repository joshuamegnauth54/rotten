use thiserror::Error;
use super::gl::{types::{GLuint, GLenum, GLint}};

#[derive(Error, Debug)]
pub enum GlError {
     #[error("shader compilation failed with {0}")]
     Shader(String),
     #[error("creating the shader program failed with {0}")]
     ShaderProgram(String)
}
