use thiserror::Error;

#[derive(Error, Debug)]
pub enum GlError {
    #[error("Buffer error: {0}")]
    Buffer(String),
    #[error("Shader compilation failed with: {0}")]
    Shader(String),
    #[error("Linking shader program failed with: {0}")]
    ShaderProgram(String),
}
