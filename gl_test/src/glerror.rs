use thiserror::Error;

#[derive(Error, Debug)]
pub enum GlError {
    #[error("Shader compilation failed with: {0}")]
    Shader(String),
    #[error("Linking shader program failed with: {0}")]
    ShaderProgram(String),
}
